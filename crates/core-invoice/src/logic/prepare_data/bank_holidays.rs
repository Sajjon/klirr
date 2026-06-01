use crate::{BankHolidays, CountryCode, Data, Date, Granularity};
use log::{debug, warn};

/// The disk-cached bank-holiday fetcher, re-exported from the foundation crate.
pub type BankHolidaysFetcher<T = ()> = klirr_foundation::BankHolidaysFetcher<T>;

/// Resolves the public holidays to deduct from billable days for an invoice,
/// degrading gracefully so holiday lookup never blocks PDF generation.
///
/// Returns an empty set (no deduction) when:
/// - `worked_holidays` is set (the per-invoice `--worked-holidays` override),
/// - the rate granularity is `Month`/`Fortnight` (holidays only affect day- and
///   hour-billed invoices),
/// - the vendor has not opted into `off_on_bank_holidays`,
/// - the vendor's free-text country cannot be mapped to an ISO country code, or
/// - the holiday API request fails (and nothing is cached).
///
/// Otherwise it returns the (possibly cached) holidays for the vendor's country
/// in the invoice period's year.
///
/// The override and granularity checks come first, so an invoice for which
/// holidays are irrelevant never triggers a country lookup or network request.
///
/// When `refresh` is `true`, a cache hit is ignored and holidays are re-fetched
/// from the API (the `--refresh-holidays` flag), picking up any corrections.
pub fn resolve_bank_holidays(
    data: &Data,
    target_period_end_date: &Date,
    worked_holidays: bool,
    refresh: bool,
) -> BankHolidays {
    if worked_holidays {
        debug!("--worked-holidays set; deducting no bank holidays for this invoice.");
        return BankHolidays::default();
    }

    let granularity = data.service_fees().rate().granularity();
    if !matches!(granularity, Granularity::Day | Granularity::Hour) {
        debug!(
            "Rate granularity {granularity:?} is unaffected by bank holidays; \
             skipping holiday resolution."
        );
        return BankHolidays::default();
    }

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
    debug!("Resolving bank holidays for {country_code} {year} (refresh: {refresh}).");
    match BankHolidaysFetcher::default().holidays_for(&country_code, year, refresh) {
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

    /// Builds invoice data with a daily rate, the given `off_on_bank_holidays`
    /// setting, and vendor country, reusing the sample vendor for everything else.
    fn data_with(off_on_bank_holidays: bool, country: &str) -> Data {
        data_with_rate(
            off_on_bank_holidays,
            country,
            crate::Rate::daily(rust_decimal::dec!(100.0)),
        )
    }

    /// As [`data_with`], but with an explicit rate so granularity can be varied.
    fn data_with_rate(off_on_bank_holidays: bool, country: &str, rate: crate::Rate) -> Data {
        let service_fees = ServiceFees::builder()
            .name("Consulting".to_string())
            .rate(rate)
            .cadence(crate::Cadence::Monthly)
            .off_on_bank_holidays(off_on_bank_holidays)
            .build()
            .unwrap();

        let vendor = crate::CompanyInformation::sample_vendor();
        let address = vendor
            .postal_address()
            .clone()
            .with_country(country.to_string());
        let vendor = vendor.with_postal_address(address);

        Data::builder()
            .information(crate::ProtoInvoiceInfo::sample())
            .vendor(vendor)
            .client(crate::CompanyInformation::sample_client())
            .payment_info(crate::PaymentInformation::sample())
            .service_fees(service_fees)
            .expensed_periods(crate::ExpensedPeriods::sample())
            .build()
    }

    #[test]
    fn disabled_flag_returns_empty() {
        // Sample data has off_on_bank_holidays = false.
        let data = Data::sample();
        let period_end = Date::sample();
        assert!(resolve_bank_holidays(&data, &period_end, false, false).is_empty());
    }

    #[test]
    fn enabled_with_unresolved_country_returns_empty_without_network() {
        // A vendor whose country cannot be mapped must degrade to empty without
        // attempting (or depending on) any network call.
        let data = data_with(true, "Atlantis");
        assert!(resolve_bank_holidays(&data, &Date::sample(), false, false).is_empty());
    }

    #[test]
    fn worked_holidays_override_wins_even_when_enabled_without_network() {
        // off_on_bank_holidays is on AND the country (Sweden) is resolvable, so
        // without the override this would hit the network. The --worked-holidays
        // override must short-circuit to empty before any country lookup/fetch.
        let data = data_with(true, "Sweden");
        assert!(resolve_bank_holidays(&data, &Date::sample(), true, false).is_empty());
    }

    #[test]
    fn refresh_alone_degrades_gracefully_for_unresolved_country() {
        // refresh=true still degrades to empty (no panic / no dependency) when
        // the country can't be resolved.
        let data = data_with(true, "Atlantis");
        assert!(resolve_bank_holidays(&data, &Date::sample(), false, true).is_empty());
    }

    #[test]
    fn monthly_granularity_skips_resolution_without_network() {
        // A fixed monthly rate is unaffected by holidays, so even with the
        // setting on and a resolvable country, no country lookup/fetch happens.
        let data = data_with_rate(
            true,
            "Sweden",
            crate::Rate::monthly(rust_decimal::dec!(50000.0)),
        );
        assert!(resolve_bank_holidays(&data, &Date::sample(), false, false).is_empty());
    }
}
