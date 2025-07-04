use crate::prelude::*;

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TypedBuilder, Getters, WithSetters)]
pub struct Data {
    /// Information about this specific invoice.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    information: ProtoInvoiceInfo,

    /// The company that issued the invoice, the vendor/seller/supplier/issuer.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    vendor: CompanyInformation,

    /// The company that pays the invoice, the customer/buyer.
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    client: CompanyInformation,

    /// Payment information for the vendor, used for international transfers.
    /// This includes the IBAN, bank name, and BIC.
    /// This is used to ensure that the client can pay the invoice correctly.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    payment_info: PaymentInformation,

    /// Price of service, if applicable.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    service_fees: ServiceFees,

    /// Any expenses that you might have incurred.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    expensed_months: ExpensedMonths,

    /// Email settings for sending the invoice.
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    email_settings: Option<EmailSettings>,
}

impl Data {
    /// Validates the invoice information and returns a `Result<Self>`.
    /// If the information is valid, it returns `Ok(self)`.
    /// If the information is invalid
    /// it returns an `Error` with the validation error.
    /// # Errors
    /// Returns an error if the invoice information is invalid.
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let data = Data::sample();
    /// let result = data.validate();
    /// assert!(result.is_ok(), "Expected validation to succeed, got: {:?}", result);
    /// ```
    pub fn validate(self) -> Result<Self> {
        self.information.validate()?;
        Ok(self)
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
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let data = Data::sample();
    /// let input = ValidInput::sample();
    /// let result = data.to_partial(input);
    /// assert!(result.is_ok(), "Expected conversion to succeed, got: {:?}", result);
    /// ```
    pub fn to_partial(self, input: ValidInput) -> Result<DataWithItemsPricedInSourceCurrency> {
        let items = input.items();
        let target_month = input.month();
        let invoice_date = target_month.to_date_end_of_month();
        let due_date = invoice_date.advance(self.payment_info().terms());
        let is_expenses = items.is_expenses();
        let number = input.invoice_number(self.information());
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
            .footer_text(self.information().footer_text().clone())
            .number(number)
            .purchase_order(self.information().purchase_order().clone())
            .build();

        let input_unpriced =
            DataFromDiskWithItemsOfKind::<LineItemsPricedInSourceCurrency>::builder()
                .client(self.client)
                .information(full_info)
                .line_items(match items {
                    InvoicedItems::Service { days_off } => {
                        let working_days = working_days_in_month(
                            target_month,
                            self.information.months_off_record(),
                        )?;
                        let worked_days = working_days - days_off.map(|d| *d).unwrap_or(0);

                        let service = Item::builder()
                            .name(self.service_fees.name())
                            .transaction_date(invoice_date)
                            .quantity(Quantity::from(Decimal::from(worked_days)))
                            .unit_price(*self.service_fees.unit_price())
                            .currency(*self.payment_info.currency())
                            .build();
                        LineItemsPricedInSourceCurrency::Service(service)
                    }
                    InvoicedItems::Expenses => {
                        let expenses = self.expensed_months.get(target_month)?;
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
            .expensed_months(ExpensedMonths::new(IndexMap::from_iter([(
                YearAndMonth::sample(),
                vec![
                    Item::sample_expense_breakfast(),
                    Item::sample_expense_coffee(),
                    Item::sample_expense_sandwich(),
                ],
            )])))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_ron_snapshot;

    use super::*;
    use test_log::test;

    #[test]
    fn test_serialization_sample() {
        assert_ron_snapshot!(Data::sample())
    }

    #[test]
    fn test_worked_days_when_ooo_is_greater_than_0() {
        let sut = Data::sample();
        let partial = sut
            .to_partial(
                ValidInput::builder()
                    .items(InvoicedItems::Service {
                        days_off: Some(Day::try_from(2).unwrap()),
                    })
                    .month(YearAndMonth::sample())
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
            &Quantity::from(dec!(20.0))
        );
    }
}
