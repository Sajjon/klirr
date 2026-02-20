use crate::{
    Cadence, CompanyInformation, DataFromDiskWithItemsOfKind, DataWithItemsPricedInSourceCurrency,
    Error, ExpensedPeriods, HasSample, InvoiceInfoFull, InvoicedItems, Item,
    LineItemsPricedInSourceCurrency, OutputPath, PaymentInformation, ProtoInvoiceInfo, Quantity,
    Result, ServiceFees, TimeOff, ValidInput, calculate_invoice_number,
    normalize_period_end_date_for_cadence, quantity_in_period,
};
use bon::Builder;
use getset::{Getters, Setters, WithSetters};
use log::debug;
use serde::{Deserialize, Serialize};

/// Input data for invoice generation.
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Builder, Getters, Setters, WithSetters,
)]
pub struct Data {
    #[getset(get = "pub", set_with = "pub")]
    information: ProtoInvoiceInfo,
    #[getset(get = "pub")]
    vendor: CompanyInformation,
    #[getset(get = "pub", set_with = "pub")]
    client: CompanyInformation,
    #[getset(get = "pub")]
    payment_info: PaymentInformation,
    #[getset(get = "pub")]
    service_fees: ServiceFees,
    #[getset(get = "pub", set = "pub")]
    expensed_periods: ExpensedPeriods,
}

impl Data {
    pub fn validate(self) -> Result<Self> {
        self.information.validate()?;
        Ok(self)
    }

    fn billable_quantity(
        &self,
        target_period_end_date: &crate::Date,
        cadence: Cadence,
        time_off: &Option<TimeOff>,
    ) -> Result<Quantity> {
        let granularity = self.service_fees().rate().granularity();
        let periods_off = self.information().record_of_periods_off();
        let quantity_in_period =
            quantity_in_period(target_period_end_date, granularity, cadence, periods_off)?;
        Ok(quantity_in_period - time_off.map(|d| *d).unwrap_or(Quantity::ZERO))
    }

    pub fn to_partial(self, input: ValidInput) -> Result<DataWithItemsPricedInSourceCurrency> {
        let items = input.items();
        let cadence = *self.service_fees().cadence();
        let target_period_end_date = normalize_period_end_date_for_cadence(*input.date(), cadence)?;
        let invoice_date = target_period_end_date;
        let due_date = invoice_date.advance(self.payment_info().terms());
        let is_expenses = items.is_expenses();

        let number = calculate_invoice_number(
            self.information().offset(),
            &target_period_end_date,
            cadence,
            is_expenses,
            self.information().record_of_periods_off(),
        )?;
        debug!(
            "calculated invoice_number: {} (cadence: {:?}, target_date: {}, expenses: {})",
            number, cadence, target_period_end_date, is_expenses
        );
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

                        let quantity =
                            self.billable_quantity(&target_period_end_date, cadence, time_off)?;
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
                        let expenses = self.expensed_periods.get(&target_period_end_date)?;
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

impl HasSample for Data {
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
    use super::*;
    use crate::{Granularity, Rate};
    use rust_decimal::dec;

    type Sut = Data;

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
    fn expenses() {
        let sut = Sut::sample();
        let input = ValidInput::builder()
            .items(InvoicedItems::Expenses)
            .date(crate::Date::sample())
            .build();
        let partial = sut.to_partial(input).unwrap();
        assert!(partial.line_items().is_expenses());
    }

    #[test]
    fn to_partial_with_free_time_with_invalid_granularity_hour_instead_of_expected_day() {
        let service_fees_hour = ServiceFees::builder()
            .name("Hourly Consulting Services".to_string())
            .rate(Rate::hourly(dec!(150.0)))
            .cadence(Cadence::Monthly)
            .build()
            .expect("Should build service fees");

        let sut = Data::builder()
            .information(ProtoInvoiceInfo::sample())
            .vendor(CompanyInformation::sample_vendor())
            .client(CompanyInformation::sample_client())
            .payment_info(PaymentInformation::sample())
            .service_fees(service_fees_hour)
            .expensed_periods(ExpensedPeriods::sample())
            .build();

        let input = ValidInput::builder()
            .items(InvoicedItems::Service {
                time_off: Some(TimeOff::Days(Quantity::from(dec!(2.0)))),
            })
            .date(crate::Date::sample())
            .build();

        let result = sut.to_partial(input);

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
}
