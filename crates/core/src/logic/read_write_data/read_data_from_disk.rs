use serde::de::DeserializeOwned;

use crate::prelude::*;

pub const BINARY_NAME: &str = "klirr";

/// Returns the path to the data directory, which is typically located at
/// ```text
/// macOS: `~/Library/Application Support/klirr/data`
/// Linux: `~/.local/share/klirr/data`
/// Windows: `C:\Users\Alice\AppData\Local\klirr\data`
/// ```
///
/// Creates if `create_if_not_exists` is true and if needed.
///
/// For more information
/// see [dirs_next][ref]
///
/// [ref]: https://docs.rs/dirs-next/latest/dirs_next/fn.data_local_dir.html
pub fn data_dir_create_if(create_if_not_exists: bool) -> PathBuf {
    let dir = dirs_next::data_local_dir()
        .expect("Should have a data directory")
        .join(BINARY_NAME)
        .join("data");
    if create_if_not_exists {
        create_folder_if_needed(&dir)
            .expect("Should be able to create directory at data_dir()/klirr/data");
    }
    dir
}

/// Returns the path to the data directory, which is typically located at
/// ```text
/// macOS: `~/Library/Application Support/klirr/data`
/// Linux: `~/.local/share/klirr/data`
/// Windows: `C:\Users\Alice\AppData\Local\klirr\data`
/// ```
///
/// For more information
/// see [dirs_next][ref]
///
/// [ref]: https://docs.rs/dirs-next/latest/dirs_next/fn.data_local_dir.html
pub fn data_dir() -> PathBuf {
    data_dir_create_if(false)
}

pub fn save_to_disk<T: Serialize>(model: &T, path: impl AsRef<Path>) -> Result<()> {
    let ron_config = ron::ser::PrettyConfig::new().struct_names(true);
    let serialized = ron::ser::to_string_pretty(model, ron_config).map_err(|e| {
        Error::FailedToRonSerializeData {
            type_name: type_name::<T>().to_owned(),
            underlying: format!("{:?}", e),
        }
    })?;
    std::fs::write(path.as_ref(), serialized).map_err(|e| Error::FailedToWriteDataToDisk {
        underlying: format!("{:?}", e),
    })?;
    info!("✅ Successfully saved file at: {}", path.as_ref().display());
    Ok(())
}
pub fn save_email_settings_with_base_path(
    email_settings: EncryptedEmailSettings,
    base_path: impl AsRef<Path>,
) -> Result<()> {
    let base_path = base_path.as_ref();
    let path = path_to_ron_file_with_base(base_path, DATA_FILE_NAME_EMAIL_SETTINGS);
    save_to_disk(&email_settings, path)
}

pub fn save_data_with_base_path<Period: IsPeriod + Serialize>(
    data: Data<Period>,
    base_path: impl AsRef<Path>,
) -> Result<()> {
    let base_path = base_path.as_ref();
    save_to_disk(data.vendor(), vendor_path(base_path))?;
    save_to_disk(data.client(), client_path(base_path))?;
    save_to_disk(data.information(), proto_invoice_info_path(base_path))?;
    save_to_disk(data.payment_info(), payment_info_path(base_path))?;
    save_to_disk(data.service_fees(), service_fees_path(base_path))?;
    save_to_disk(data.expensed_periods(), expensed_periods_path(base_path))?;
    Ok(())
}

pub fn path_to_ron_file_with_base(base_path: impl AsRef<Path>, name: &str) -> PathBuf {
    base_path.as_ref().join(format!("{}.ron", name))
}

pub fn load_data<T: DeserializeOwned>(base_path: impl AsRef<Path>, name: &str) -> Result<T> {
    deserialize_contents_of_ron(path_to_ron_file_with_base(base_path, name))
}

const DATA_FILE_NAME_EMAIL_SETTINGS: &str = "email";
const DATA_FILE_NAME_VENDOR: &str = "vendor";
const DATA_FILE_NAME_CLIENT: &str = "client";
const DATA_FILE_NAME_PAYMENT: &str = "payment";
const DATA_FILE_NAME_SERVICE_FEES: &str = "service_fees";
const DATA_FILE_NAME_PROTO_INVOICE_INFO: &str = "invoice_info";
const DATA_FILE_NAME_EXPENSES: &str = "expenses";
const DATA_FILE_NAME_CACHED_RATES: &str = "cached_rates";

pub fn email_settings_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_EMAIL_SETTINGS)
}

pub fn cached_rates_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_CACHED_RATES)
}

pub fn client_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_CLIENT)
}

pub fn vendor_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_VENDOR)
}

pub fn payment_info_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_PAYMENT)
}

pub fn service_fees_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_SERVICE_FEES)
}

pub fn proto_invoice_info_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_PROTO_INVOICE_INFO)
}

pub fn expensed_periods_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_EXPENSES)
}

fn client(base_path: impl AsRef<Path>) -> Result<CompanyInformation> {
    deserialize_contents_of_ron(client_path(base_path))
}

fn vendor(base_path: impl AsRef<Path>) -> Result<CompanyInformation> {
    deserialize_contents_of_ron(vendor_path(base_path))
}

fn payment_info(base_path: impl AsRef<Path>) -> Result<PaymentInformation> {
    deserialize_contents_of_ron(payment_info_path(base_path))
}

pub fn service_fees(base_path: impl AsRef<Path>) -> Result<ServiceFees> {
    deserialize_contents_of_ron(service_fees_path(base_path))
}

pub fn proto_invoice_info<Period: IsPeriod + DeserializeOwned>(
    base_path: impl AsRef<Path>,
) -> Result<ProtoInvoiceInfo<Period>> {
    deserialize_contents_of_ron(proto_invoice_info_path(base_path))
}

pub fn expensed_periods<Period: IsPeriod + DeserializeOwned>(
    base_path: impl AsRef<Path>,
) -> Result<ExpensedPeriods<Period>> {
    deserialize_contents_of_ron(expensed_periods_path(base_path))
}

pub fn read_email_data_from_disk_with_base_path(
    base_path: impl AsRef<Path>,
) -> Result<EncryptedEmailSettings> {
    deserialize_contents_of_ron(email_settings_path(base_path))
}

pub fn read_data_from_disk_with_base_path(base_path: impl AsRef<Path>) -> Result<Data<PeriodAnno>> {
    let base_path = base_path.as_ref();
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    debug!("☑️ Reading data from disk...");
    let client = client(base_path)?;
    let vendor = vendor(base_path)?;
    let payment_info = payment_info(base_path)?;
    let service_fees = service_fees(base_path)?;
    let proto_invoice_info = proto_invoice_info(base_path)?;
    let expensed_periods = expensed_periods(base_path)?;

    let input_data = Data::builder()
        .client(client)
        .vendor(vendor)
        .payment_info(payment_info)
        .service_fees(service_fees)
        .information(proto_invoice_info)
        .expensed_periods(expensed_periods)
        .build();
    debug!("✅ Read data from disk!");
    input_data.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn write_read_validate_data() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let data = Data::<PeriodAnno>::sample();
        save_data_with_base_path(data.clone(), tempdir.path()).unwrap();
        let loaded_data = read_data_from_disk_with_base_path(tempdir.path()).unwrap();
        assert_eq!(loaded_data, data, "Loaded data should match saved data");
    }
}
