use crate::{
    Cadence, CompanyInformation, DataFromDiskWithItemsOfKind, DataWithItemsPricedInSourceCurrency,
    Error, ExpensedPeriods, HasSample, InvoiceInfoFull, InvoicedItems, IsPeriod, Item,
    LineItemsPricedInSourceCurrency, OutputPath, PaymentInformation, PeriodAnno, ProtoInvoiceInfo,
    Quantity, Result, ServiceFees, TimeOff, ValidInput, calculate_invoice_number,
    quantity_in_period,
};
use bon::Builder;
use getset::Getters;
use getset::Setters;
use getset::WithSetters;
use serde::Deserialize;
use serde::Serialize;

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Builder, Getters, Setters, WithSetters,
)]
pub struct Data<Period: IsPeriod> {
    /// Information about this specific invoice.
    #[getset(get = "pub", set_with = "pub")]
    information: ProtoInvoiceInfo<Period>,

    /// The company that issued the invoice, the vendor/seller/supplier/issuer.
    #[getset(get = "pub")]
    vendor: CompanyInformation,

    /// The company that pays the invoice, the customer/buyer.
    #[getset(get = "pub", set_with = "pub")]
    client: CompanyInformation,

    /// Payment information for the vendor, used for international transfers.
    /// This includes the IBAN, bank name, and BIC.
    /// This is used to ensure that the client can pay the invoice correctly.
    #[getset(get = "pub")]
    payment_info: PaymentInformation,

    /// Price of service, if applicable.
    #[getset(get = "pub")]
    service_fees: ServiceFees,

    /// Any expenses that you might have incurred.
    #[getset(get = "pub", set = "pub")]
    expensed_periods: ExpensedPeriods<Period>,
}

impl<Period: IsPeriod> Data<Period> {
    /// Validates the invoice information and returns a `Result<Self>`.
    /// If the information is valid, it returns `Ok(self)`.
    /// If the information is invalid
    /// it returns an `Error` with the validation error.
    /// # Errors
    /// Returns an error if the invoice information is invalid.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let data = Data::<YearAndMonth>::sample();
    /// let result = data.validate();
    /// assert!(result.is_ok(), "Expected validation to succeed, got: {:?}", result);
    /// ```
    pub fn validate(self) -> Result<Self> {
        self.information.validate()?;
        Ok(self)
    }

    fn billable_quantity(
        &self,
        target_period: &Period,
        cadence: Cadence,
        time_off: &Option<TimeOff>,
    ) -> Result<Quantity> {
        let granularity = self.service_fees().rate().granularity();
        let periods_off = self.information().record_of_periods_off();
        let quantity_in_period =
            quantity_in_period(target_period, granularity, cadence, periods_off)?;
        let billable_quantity = quantity_in_period - time_off.map(|d| *d).unwrap_or(Quantity::ZERO);
        Ok(billable_quantity)
    }

    /// Converts the `Data` into a `DataWithItemsPricedInSourceCurrency`
    /// using the provided `ValidInput`.
    /// This method prepares the invoice data for rendering by creating an
    /// `InvoiceInfoFull` and populating it with the necessary information.
    /// It also calculates the invoice date, due date, and prepares the output path.
    /// # Errors
    /// Returns an error if the month is invalid or if there are issues with
    /// retrieving expenses for the month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let data = Data::<YearAndMonth>::sample();
    /// let input = ValidInput::sample();
    /// let result = data.to_partial(input);
    /// assert!(result.is_ok(), "Expected conversion to succeed, got: {:?}", result);
    /// ```
    pub fn to_partial(self, input: ValidInput) -> Result<DataWithItemsPricedInSourceCurrency> {
        let input_period = input.period().clone();
        match (*self.service_fees().cadence(), &input_period) {
            (Cadence::Monthly, PeriodAnno::YearMonthAndFortnight(_)) => {
                return Err(Error::CannotInvoiceForFortnightWhenCadenceIsMonthly);
            }
            (Cadence::BiWeekly, PeriodAnno::YearAndMonth(_)) => {
                return Err(Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly);
            }
            _ => {}
        }
        let target_period = Period::try_from_period_anno(input_period)?;
        let items = input.items();
        let invoice_date = target_period.to_date_end_of_period();
        let due_date = invoice_date.advance(self.payment_info().terms());
        let is_expenses = items.is_expenses();

        let number = calculate_invoice_number(
            self.information().offset(),
            &target_period,
            is_expenses,
            self.information().record_of_periods_off(),
        )?;
        let is_expenses_str_or_empty = if is_expenses { "_expenses" } else { "" };
        let vendor_name = self.vendor.company_name().replace(' ', "_");

        let output_path = input
            .maybe_output_path()
            .as_ref()
            .cloned()
            .map(OutputPath::AbsolutePath)
            .unwrap_or_else(|| {
                OutputPath::Name(format!(
                    "{}_{}{}_invoice_{}.pdf",
                    invoice_date, vendor_name, is_expenses_str_or_empty, number
                ))
            });

        let full_info = InvoiceInfoFull::builder()
            .due_date(due_date)
            .invoice_date(invoice_date)
            .emphasize_color_hex(
                self.information()
                    .emphasize_color_hex()
                    .clone()
                    .unwrap_or_default(),
            )
            .maybe_footer_text(self.information().footer_text().clone())
            .number(number)
            .maybe_purchase_order(self.information().purchase_order().clone())
            .build();

        let input_unpriced =
            DataFromDiskWithItemsOfKind::<LineItemsPricedInSourceCurrency>::builder()
                .client(self.client.clone())
                .information(full_info)
                .line_items(match items {
                    InvoicedItems::Service { time_off } => {
                        if let Some(time_off) = time_off {
                            if time_off.granularity() != self.service_fees().rate().granularity() {
                                return Err(Error::InvalidGranularityForTimeOff {
                                    free_granularity: time_off.granularity(),
                                    service_fees_granularity: self
                                        .service_fees()
                                        .rate()
                                        .granularity(),
                                });
                            }
                        }

                        let quantity = self.billable_quantity(
                            &target_period,
                            *self.service_fees().cadence(),
                            time_off,
                        )?;
                        let service = Item::builder()
                            .name(self.service_fees.name().clone())
                            .transaction_date(invoice_date)
                            .quantity(quantity)
                            .unit_price(self.service_fees.unit_price())
                            .currency(*self.payment_info.currency())
                            .build();
                        LineItemsPricedInSourceCurrency::Service(service)
                    }
                    InvoicedItems::Expenses => {
                        let expenses = self.expensed_periods.get(&target_period)?;
                        LineItemsPricedInSourceCurrency::Expenses(expenses.clone())
                    }
                })
                .payment_info(self.payment_info)
                .vendor(self.vendor)
                .output_path(output_path)
                .build();

        Ok(input_unpriced)
    }
}

impl<Period: IsPeriod + HasSample> HasSample for Data<Period> {
    fn sample() -> Self {
        Data::builder()
            .information(ProtoInvoiceInfo::sample())
            .client(CompanyInformation::sample_client())
            .vendor(CompanyInformation::sample_vendor())
            .payment_info(PaymentInformation::sample())
            .service_fees(ServiceFees::sample())
            .expensed_periods(ExpensedPeriods::sample())
            .build()
    }

    fn sample_other() -> Self {
        Data::builder()
            .information(ProtoInvoiceInfo::sample_other())
            .client(CompanyInformation::sample_client())
            .vendor(CompanyInformation::sample_vendor())
            .payment_info(PaymentInformation::sample_other())
            .service_fees(ServiceFees::sample_other())
            .expensed_periods(ExpensedPeriods::sample_other())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_ron_snapshot;

    use super::*;
    use crate::HasSample;
    use crate::{
        Granularity, Month, MonthHalf, Rate, TimestampedInvoiceNumber, YearAndMonth,
        YearMonthAndFortnight,
    };
    use rust_decimal::dec;
    use test_log::test;

    type Sut = Data<YearAndMonth>;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn test_serialization_sample() {
        assert_ron_snapshot!(Sut::sample())
    }

    #[test]
    fn expenses() {
        let sut = Sut::sample();
        let input = ValidInput::builder()
            .items(InvoicedItems::Expenses)
            .period(YearAndMonth::may(2025).into())
            .build();
        let partial = sut.to_partial(input).unwrap();
        assert!(partial.line_items().is_expenses());
    }

    #[test]
    fn to_partial_rejects_fortnight_when_cadence_monthly() {
        let sut = Sut::sample();
        let input = ValidInput::builder()
            .items(InvoicedItems::Service { time_off: None })
            .period(YearMonthAndFortnight::sample().into())
            .build();
        let result = sut.to_partial(input);
        assert_eq!(
            result.unwrap_err(),
            Error::CannotInvoiceForFortnightWhenCadenceIsMonthly
        );
    }

    #[test]
    fn test_worked_days_when_ooo_is_greater_than_0() {
        let sut = Sut::sample();
        let partial = sut
            .to_partial(
                ValidInput::builder()
                    .items(InvoicedItems::Service {
                        time_off: Some(TimeOff::Days(Quantity::from(dec!(2.0)))),
                    })
                    .period(YearAndMonth::sample().into())
                    .build(),
            )
            .unwrap();
        assert_eq!(
            partial
                .line_items()
                .clone()
                .try_unwrap_service()
                .unwrap()
                .quantity(),
            &Quantity::from(dec!(21.0))
        );
    }
    #[test]
    fn to_partial_with_free_time_with_invalid_granularity_hour_instead_of_expected_day() {
        // Create service fees with Hour granularity (more granular than Day)
        let service_fees_hour = ServiceFees::builder()
            .name("Hourly Consulting Services".to_string())
            .rate(Rate::hourly(dec!(150.0)))
            .cadence(Cadence::Monthly)
            .build()
            .expect("Should build service fees");

        // Create data with Hour granularity service fees
        let sut = Data::<YearAndMonth>::builder()
            .information(ProtoInvoiceInfo::sample())
            .vendor(CompanyInformation::sample_vendor())
            .client(CompanyInformation::sample_client())
            .payment_info(PaymentInformation::sample())
            .service_fees(service_fees_hour)
            .expensed_periods(ExpensedPeriods::sample())
            .build();

        let input = ValidInput::builder()
            .items(InvoicedItems::Service {
                // Free time is Day granularity, but service is Hour granularity
                // Day > Hour in the granularity ordering, so this should fail
                time_off: Some(TimeOff::Days(Quantity::from(dec!(2.0)))),
            })
            .period(YearAndMonth::sample().into())
            .build();

        let result = sut.to_partial(input);

        assert!(
            result.is_err(),
            "Expected InvalidGranularityForTimeOff error"
        );

        if let Err(Error::InvalidGranularityForTimeOff {
            free_granularity,
            service_fees_granularity,
        }) = result
        {
            assert_eq!(free_granularity, Granularity::Day);
            assert_eq!(service_fees_granularity, Granularity::Hour);
        } else {
            panic!(
                "Expected InvalidGranularityForTimeOff error, got: {:?}",
                result
            );
        }
    }

    #[test]
    fn to_partial_with_free_time_with_invalid_granularity_hour_for_day_service() {
        // Create service fees with Day granularity (less granular than Hour)
        let service_fees_day = ServiceFees::builder()
            .name("Daily Consulting Services".to_string())
            .rate(Rate::daily(dec!(1000.0)))
            .cadence(Cadence::Monthly)
            .build()
            .expect("Should build service fees");

        // Create data with Day granularity service fees
        let sut = Data::<YearAndMonth>::builder()
            .information(ProtoInvoiceInfo::sample())
            .vendor(CompanyInformation::sample_vendor())
            .client(CompanyInformation::sample_client())
            .payment_info(PaymentInformation::sample())
            .service_fees(service_fees_day)
            .expensed_periods(ExpensedPeriods::sample())
            .build();

        let input = ValidInput::builder()
            .items(InvoicedItems::Service {
                // Free time is Hour granularity, service is Day granularity
                // Hour < Day in the granularity ordering, so this should succeed
                // because free time can be more granular than service granularity
                time_off: Some(TimeOff::Hours(Quantity::from(dec!(8.0)))),
            })
            .period(YearAndMonth::sample().into())
            .build();

        let result = sut.to_partial(input);

        // Should fail with InvalidGranularityForTimeOff error
        assert!(
            result.is_err(),
            "Expected InvalidGranularityForTimeOff error"
        );

        if let Err(Error::InvalidGranularityForTimeOff {
            free_granularity,
            service_fees_granularity,
        }) = result
        {
            assert_eq!(free_granularity, Granularity::Hour);
            assert_eq!(service_fees_granularity, Granularity::Day);
        } else {
            panic!(
                "Expected InvalidGranularityForTimeOff error, got: {:?}",
                result
            );
        }
    }

    #[test]
    fn test_to_partial_when_offset_is_year_month_and_fortnight() {
        let offset_period = YearMonthAndFortnight::builder()
            .year(2025)
            .month(Month::May)
            .half(MonthHalf::First)
            .build();
        let sut = Data::<YearMonthAndFortnight>::builder()
            .information(
                ProtoInvoiceInfo::builder()
                    .offset(
                        TimestampedInvoiceNumber::<YearMonthAndFortnight>::builder()
                            .offset(100)
                            .period(offset_period)
                            .build(),
                    )
                    .build(),
            )
            .vendor(CompanyInformation::sample_vendor())
            .client(CompanyInformation::sample_client())
            .payment_info(PaymentInformation::sample())
            .service_fees(ServiceFees::sample_other())
            .expensed_periods(ExpensedPeriods::sample())
            .build();
        let target_period = YearMonthAndFortnight::builder()
            .year(2025)
            .month(Month::May)
            .half(MonthHalf::First)
            .build();
        let input = ValidInput::builder()
            .items(InvoicedItems::Service { time_off: None })
            .period(target_period.into())
            .build();
        let partial = sut.to_partial(input).unwrap();
        let invoice_date = partial.information().invoice_date();
        assert_eq!(*invoice_date, target_period.to_date_end_of_period());
    }
}
