pub const INVOICES_FOLDER_NAME: &str = "invoices";

use crate::prelude::*;

pub type DataWithItemsPricedInSourceCurrency =
    DataFromDiskWithItemsOfKind<LineItemsPricedInSourceCurrency>;
pub type PreparedData = DataFromDiskWithItemsOfKind<LineItemsFlat>;

impl ToTypst for PreparedData {}

pub trait HasSample: Sized {
    /// Returns a sample instance of the type.
    fn sample() -> Self;
}

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct DataFromDiskWithItemsOfKind<Items: Serialize + MaybeIsExpenses> {
    /// Information about this specific invoice.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    information: InvoiceInfoFull,

    /// The company that issued the invoice, the vendor/seller/supplier/issuer.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    vendor: CompanyInformation,

    /// The company that pays the invoice, the customer/buyer.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    client: CompanyInformation,

    /// Services or expenses included in this invoice to be paid by the client.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    line_items: Items,

    /// Payment information for the vendor, used for international transfers.
    /// This includes the IBAN, bank name, and BIC.
    /// This is used to ensure that the client can pay the invoice correctly.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    payment_info: PaymentInformation,

    /// Where to save the output PDF file.
    #[builder(setter(into))]
    output_path: OutputPath,
}

impl<Items: Serialize + MaybeIsExpenses> DataFromDiskWithItemsOfKind<Items> {
    /// Returns the absolute path where the invoice will be saved.
    /// If the path is relative, it will be created relative to the workspace root.
    /// If the path is absolute, it will be returned as is.
    ///
    /// # Errors
    /// Returns an error if the path is relative and the folder cannot be created,
    /// or if the path is absolute but invalid.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let data = DataWithItemsPricedInSourceCurrency::builder()
    ///    .information(InvoiceInfoFull::sample())
    ///   .vendor(CompanyInformation::sample())
    ///   .client(CompanyInformation::sample())
    ///   .line_items(LineItemsPricedInSourceCurrency::sample())
    ///   .payment_info(PaymentInformation::sample())
    ///   .output_path(OutputPath::Name("invoice.pdf".into()))
    ///   .build();
    ///
    /// let absolute_path = data.absolute_path().unwrap();
    /// assert!(absolute_path.ends_with("invoice.pdf"));
    /// ```
    pub fn absolute_path(&self) -> Result<PathBuf> {
        match &self.output_path {
            OutputPath::AbsolutePath(path) => Ok(path.clone()),
            OutputPath::Name(name) => {
                let mut path =
                    dirs_next::home_dir().ok_or(Error::FailedToCreateOutputDirectory {
                        underlying: "Failed to find output dir (home dir)".to_owned(),
                    })?;
                path.push(INVOICES_FOLDER_NAME);
                path.push(name);
                Ok(path)
            }
        }
    }
}

impl<Items: Serialize + MaybeIsExpenses + HasSample> HasSample
    for DataFromDiskWithItemsOfKind<Items>
{
    fn sample() -> Self {
        Self::builder()
            .information(InvoiceInfoFull::sample())
            .vendor(CompanyInformation::sample())
            .client(CompanyInformation::sample())
            .line_items(Items::sample())
            .payment_info(PaymentInformation::sample())
            .output_path(OutputPath::Name("invoice.pdf".into()))
            .build()
    }
}

impl DataWithItemsPricedInSourceCurrency {
    /// Converts the `DataWithItemsPricedInSourceCurrency` into a `PreparedData`
    /// which is compatible with Typst rendering.
    /// This method prepares the invoice data for rendering by creating an
    /// `ExchangeRates` object and converting the line items into a flat structure.
    ///
    /// # Errors
    /// Returns an error if the line items cannot be converted to a flat structure.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let data = DataWithItemsPricedInSourceCurrency::sample();
    /// let exchange_rates = ExchangeRates::builder().rates(HashMap::from_iter([(Currency::GBP, UnitPrice::from(10.0)), (Currency::EUR, UnitPrice::from(8.0))])).target_currency(Currency::EUR).build();
    /// let result = data.to_typst(exchange_rates);
    /// assert!(result.is_ok(), "Expected conversion to succeed, got: {:?}", result);
    /// ```
    pub fn to_typst(self, exchange_rates: ExchangeRates) -> Result<PreparedData> {
        let line_items = LineItemsFlat::try_from((self.line_items, exchange_rates))?;
        Ok(PreparedData {
            line_items,
            information: self.information,
            vendor: self.vendor,
            client: self.client,
            payment_info: self.payment_info,
            output_path: self.output_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_get_absolute_path_with_name() {
        let data = DataWithItemsPricedInSourceCurrency::builder()
            .output_path(OutputPath::Name("invoice.pdf".to_owned()))
            .information(InvoiceInfoFull::sample())
            .vendor(CompanyInformation::sample())
            .client(CompanyInformation::sample())
            .payment_info(PaymentInformation::sample())
            .line_items(LineItemsPricedInSourceCurrency::sample())
            .build();
        assert!(
            data.absolute_path()
                .unwrap()
                .ends_with("invoices/invoice.pdf")
        );
    }

    #[test]
    fn test_get_absolute_path_with_absolute_path() {
        let custom_path = PathBuf::from("custom/invoice.pdf");
        let data = DataWithItemsPricedInSourceCurrency::builder()
            .output_path(OutputPath::AbsolutePath(custom_path.clone()))
            .information(InvoiceInfoFull::sample())
            .vendor(CompanyInformation::sample())
            .client(CompanyInformation::sample())
            .payment_info(PaymentInformation::sample())
            .line_items(LineItemsPricedInSourceCurrency::sample())
            .build();
        assert_eq!(data.absolute_path().unwrap(), custom_path);
    }

    #[test]
    fn sample_data_from_disk_with_items_of_kind() {
        let data = DataFromDiskWithItemsOfKind::<LineItemsPricedInSourceCurrency>::sample();
        assert_eq!(data.output_path, OutputPath::Name("invoice.pdf".into()));
    }
}
