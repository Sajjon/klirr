pub const INVOICES_FOLDER_NAME: &str = "invoices";

use klirr_foundation::ToTypst;

use crate::prelude::*;

pub type DataWithItemsPricedInSourceCurrency =
    DataFromDiskWithItemsOfKind<LineItemsPricedInSourceCurrency>;
pub type PreparedData = DataFromDiskWithItemsOfKind<LineItemsFlat>;

impl ToTypst for PreparedData {}

pub trait HasSample: Sized {
    /// Returns a sample instance of the type.
    fn sample() -> Self;
    fn sample_other() -> Self;
}

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Builder, Getters)]
pub struct DataFromDiskWithItemsOfKind<Items: Serialize + MaybeIsExpenses> {
    /// Information about this specific invoice.
    #[getset(get = "pub")]
    information: InvoiceInfoFull,

    /// The company that issued the invoice, the vendor/seller/supplier/issuer.
    #[getset(get = "pub")]
    vendor: CompanyInformation,

    /// The company that pays the invoice, the customer/buyer.
    #[getset(get = "pub")]
    client: CompanyInformation,

    /// Services or expenses included in this invoice to be paid by the client.
    #[getset(get = "pub")]
    line_items: Items,

    /// Payment information for the vendor, used for international transfers.
    /// This includes the IBAN, bank name, and BIC.
    /// This is used to ensure that the client can pay the invoice correctly.
    #[getset(get = "pub")]
    payment_info: PaymentInformation,

    /// Where to save the output PDF file.
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
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let data = DataWithItemsPricedInSourceCurrency::builder()
    ///    .information(InvoiceInfoFull::sample())
    ///   .vendor(CompanyInformation::sample())
    ///   .client(CompanyInformation::sample())
    ///   .line_items(LineItemsPricedInSourceCurrency::sample())
    ///   .payment_info(PaymentInformation::sample())
    ///   .output_path(OutputPath::Name("invoice.pdf".into()))
    ///   .build();
    ///
    /// let path_and_name = data.absolute_path_and_name().unwrap();
    /// assert!(path_and_name.path().ends_with("invoice.pdf"));
    /// assert_eq!(path_and_name.name(), "invoice.pdf");
    /// ```
    pub fn absolute_path_and_name(&self) -> Result<PathAndName> {
        match &self.output_path {
            OutputPath::AbsolutePath(path) => Ok(PathAndName::builder()
                .path(path.clone())
                .name(path.file_name().unwrap().to_string_lossy().into())
                .build()),
            OutputPath::Name(name) => {
                let mut path =
                    dirs_next::home_dir().ok_or(Error::FailedToCreateOutputDirectory {
                        underlying: "Failed to find output dir (home dir)".to_owned(),
                    })?;
                path.push(INVOICES_FOLDER_NAME);
                path.push(name);
                Ok(PathAndName::builder().path(path).name(name.clone()).build())
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Builder, Getters)]
pub struct PathAndName {
    /// The absolute path to the file.
    #[getset(get = "pub")]
    path: PathBuf,

    /// The name of the file.
    #[getset(get = "pub")]
    name: String,
}

impl<Items: Serialize + MaybeIsExpenses + HasSample> HasSample
    for DataFromDiskWithItemsOfKind<Items>
{
    fn sample() -> Self {
        Self::builder()
            .information(InvoiceInfoFull::sample())
            .vendor(CompanyInformation::sample_vendor())
            .client(CompanyInformation::sample_client())
            .line_items(Items::sample())
            .payment_info(PaymentInformation::sample())
            .output_path(OutputPath::Name("invoice.pdf".into()))
            .build()
    }
    fn sample_other() -> Self {
        Self::builder()
            .information(InvoiceInfoFull::sample_other())
            .vendor(CompanyInformation::sample_client())
            .client(CompanyInformation::sample_vendor())
            .line_items(Items::sample_other())
            .payment_info(PaymentInformation::sample_other())
            .output_path(OutputPath::Name("invoice_other.pdf".into()))
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
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let data = DataWithItemsPricedInSourceCurrency::sample();
    /// let exchange_rates = ExchangeRates::builder().rates(ExchangeRatesMap::from_iter([(Currency::GBP, UnitPrice::from(dec!(10.0))), (Currency::EUR, UnitPrice::from(dec!(8.0)))])).target_currency(Currency::EUR).build();
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
        let path_and_name = data.absolute_path_and_name().unwrap();
        assert!(path_and_name.path().ends_with("invoices/invoice.pdf"));
        assert!(path_and_name.name().ends_with("invoice.pdf"));
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
        let path_and_name = data.absolute_path_and_name().unwrap();
        assert_eq!(path_and_name.path(), &custom_path);
        assert_eq!(path_and_name.name(), "invoice.pdf");
    }

    #[test]
    fn sample_data_from_disk_with_items_of_kind() {
        let data = DataFromDiskWithItemsOfKind::<LineItemsPricedInSourceCurrency>::sample();
        assert_eq!(data.output_path, OutputPath::Name("invoice.pdf".into()));
    }
}
