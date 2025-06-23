use serde::de::DeserializeOwned;

use crate::prelude::*;

pub fn input_dir() -> PathBuf {
    dirs_next::data_dir()
        .expect("Should have a data directory")
        .join("klirr/input")
}

pub fn data_dir() -> PathBuf {
    input_dir().join("data")
}

pub fn l18n_dir() -> PathBuf {
    input_dir().join("l18n")
}

pub fn data_path_ron_file(name: &str) -> PathBuf {
    data_dir().join(format!("{}.ron", name))
}

pub fn load_data<T: DeserializeOwned>(name: &str) -> Result<T> {
    deserialize_contents_of_ron(data_path_ron_file(name))
}

pub const DATA_FILE_NAME_VENDOR: &str = "vendor";
pub const DATA_FILE_NAME_CLIENT: &str = "client";
pub const DATA_FILE_NAME_PAYMENT: &str = "payment";
pub const DATA_FILE_NAME_SERVICE_FEES: &str = "service_fees";
pub const DATA_FILE_NAME_PROTO_INVOICE_INFO: &str = "invoice_info";
pub const DATA_FILE_NAME_EXPENSES: &str = "expenses";

fn client() -> Result<CompanyInformation> {
    load_data(DATA_FILE_NAME_CLIENT)
}

fn vendor() -> Result<CompanyInformation> {
    load_data(DATA_FILE_NAME_VENDOR)
}

fn payment_info() -> Result<PaymentInformation> {
    load_data(DATA_FILE_NAME_PAYMENT)
}

fn service_fees() -> Result<ServiceFees> {
    load_data(DATA_FILE_NAME_SERVICE_FEES)
}

fn proto_invoice_info() -> Result<ProtoInvoiceInfo> {
    load_data(DATA_FILE_NAME_PROTO_INVOICE_INFO)
}

fn expensed_months() -> Result<ExpensedMonths> {
    load_data(DATA_FILE_NAME_EXPENSES)
}

pub fn read_data_from_disk() -> Result<DataFromDisk> {
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    debug!("☑️ Reading data from disk...");
    let client = client()?;
    let vendor = vendor()?;
    let payment_info = payment_info()?;
    let service_prices = service_fees()?;
    let proto_invoice_info = proto_invoice_info()?;
    let expensed_months = expensed_months()?;

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
