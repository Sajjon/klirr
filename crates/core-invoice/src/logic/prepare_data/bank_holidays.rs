use crate::{BankHolidays, CountryCode, Data, Date};
use log::{debug, warn};

/// The disk-cached bank-holiday fetcher, re-exported from the foundation crate.
pub type BankHolidaysFetcher<T = ()> = klirr_foundation::BankHolidaysFetcher<T>;

/// Resolves the public holidays to deduct from billable days for an invoice,
/// degrading gracefully so holiday lookup never blocks PDF generation.
///
/// Returns an empty set (no deduction) when:
/// - the vendor has not opted into `off_on_bank_holidays`,
/// - the vendor's free-text country cannot be mapped to an ISO country code, or
/// - the holiday API request fails (and nothing is cached).
///
/// Otherwise it returns the (possibly cached) holidays for the vendor's country
/// in the invoice period's year.
pub fn resolve_bank_holidays(data: &Data, target_period_end_date: &Date) -> BankHolidays {
    if !data.service_fees().off_on_bank_holidays() {
        return BankHolidays::default();
    }

    let country_name = data.vendor().postal_address().country();
    let Some(country_code) = CountryCode::from_country_name(country_name) else {
        warn!(
            "off_on_bank_holidays is enabled but vendor country '{country_name}' could not be \
             resolved to an ISO country code; skipping bank-holiday deduction."
        );
        return BankHolidays::default();
    };

    let year = *target_period_end_date.year();
    debug!("Resolving bank holidays for {country_code} {year}.");
    match BankHolidaysFetcher::default().holidays_for(&country_code, year) {
        Ok(holidays) => holidays,
        Err(error) => {
            warn!(
                "Failed to fetch bank holidays for {country_code} {year}: {error}. \
                 Skipping bank-holiday deduction."
            );
            BankHolidays::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HasSample, ServiceFees};
    use test_log::test;

    #[test]
    fn disabled_flag_returns_empty() {
        // Sample data has off_on_bank_holidays = false.
        let data = Data::sample();
        let period_end = Date::sample();
        assert!(resolve_bank_holidays(&data, &period_end).is_empty());
    }

    #[test]
    fn enabled_with_unresolved_country_returns_empty_without_network() {
        // A vendor whose country cannot be mapped must degrade to empty without
        // attempting (or depending on) any network call.
        let service_fees = ServiceFees::builder()
            .name("Consulting".to_string())
            .rate(crate::Rate::daily(rust_decimal::dec!(100.0)))
            .cadence(crate::Cadence::Monthly)
            .off_on_bank_holidays(true)
            .build()
            .unwrap();

        let vendor = crate::CompanyInformation::sample_vendor();
        let address = vendor
            .postal_address()
            .clone()
            .with_country("Atlantis".to_string());
        let vendor = vendor.with_postal_address(address);

        let data = Data::builder()
            .information(crate::ProtoInvoiceInfo::sample())
            .vendor(vendor)
            .client(crate::CompanyInformation::sample_client())
            .payment_info(crate::PaymentInformation::sample())
            .service_fees(service_fees)
            .expensed_periods(crate::ExpensedPeriods::sample())
            .build();

        assert!(resolve_bank_holidays(&data, &Date::sample()).is_empty());
    }
}
