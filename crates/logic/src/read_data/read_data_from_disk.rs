use serde::de::DeserializeOwned;

use crate::prelude::*;

pub fn input_dir() -> PathBuf {
    dirs_next::data_dir()
        .expect("Should have a data directory")
        .join("inrost/input")
}

pub fn data_dir() -> PathBuf {
    input_dir().join("data")
}

pub fn l18n_dir() -> PathBuf {
    input_dir().join("l18n")
}

pub fn data_path_ron_file(base_path: impl Into<Option<PathBuf>>, name: &str) -> PathBuf {
    let path = base_path.into().unwrap_or_else(data_dir);
    path.join(format!("{}.ron", name))
}

pub fn load_data<T: DeserializeOwned>(
    base_path: impl Into<Option<PathBuf>>,
    name: &str,
) -> Result<T> {
    deserialize_contents_of_ron(data_path_ron_file(base_path, name))
}

pub const DATA_FILE_NAME_VENDOR: &str = "vendor";
pub const DATA_FILE_NAME_CLIENT: &str = "client";
pub const DATA_FILE_NAME_PAYMENT: &str = "payment";
pub const DATA_FILE_NAME_SERVICE_FEES: &str = "service_fees";
pub const DATA_FILE_NAME_PROTO_INVOICE_INFO: &str = "invoice_info";
pub const DATA_FILE_NAME_EXPENSES: &str = "expenses";

fn client(base_path: impl Into<Option<PathBuf>>) -> Result<CompanyInformation> {
    load_data(base_path, DATA_FILE_NAME_CLIENT)
}

fn vendor(base_path: impl Into<Option<PathBuf>>) -> Result<CompanyInformation> {
    load_data(base_path, DATA_FILE_NAME_VENDOR)
}

fn payment_info(base_path: impl Into<Option<PathBuf>>) -> Result<PaymentInformation> {
    load_data(base_path, DATA_FILE_NAME_PAYMENT)
}

fn service_fees(base_path: impl Into<Option<PathBuf>>) -> Result<ServiceFees> {
    load_data(base_path, DATA_FILE_NAME_SERVICE_FEES)
}

fn proto_invoice_info(base_path: impl Into<Option<PathBuf>>) -> Result<ProtoInvoiceInfo> {
    load_data(base_path, DATA_FILE_NAME_PROTO_INVOICE_INFO)
}

fn expensed_months(base_path: impl Into<Option<PathBuf>>) -> Result<ExpensedMonths> {
    load_data(base_path, DATA_FILE_NAME_EXPENSES)
}

pub fn read_data_from_disk() -> Result<DataFromDisk> {
    read_data_from_disk_base_dir(None) // use default
}
pub fn read_data_from_disk_base_dir(base_path: impl Into<Option<PathBuf>>) -> Result<DataFromDisk> {
    let base_path = base_path.into();
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    debug!("☑️ Reading data from disk...");
    let client = client(base_path.clone())?;
    let vendor = vendor(base_path.clone())?;
    let payment_info = payment_info(base_path.clone())?;
    let service_prices = service_fees(base_path.clone())?;
    let proto_invoice_info = proto_invoice_info(base_path.clone())?;
    let expensed_months = expensed_months(base_path.clone())?;

    let input_data = DataFromDisk::builder()
        .client(client)
        .vendor(vendor)
        .payment_info(payment_info)
        .service_fees(service_prices)
        .information(proto_invoice_info)
        .expensed_months(expensed_months)
        .build();
    debug!("✅ Read data from disk!");
    input_data.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_read_data_from_disk() {
        let result = read_data_from_disk().unwrap();
        assert_eq!(*result.payment_info().currency(), Currency::EUR);
    }
}
