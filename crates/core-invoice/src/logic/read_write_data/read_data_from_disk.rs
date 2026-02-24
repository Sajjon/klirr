use serde::de::DeserializeOwned;

use crate::deserialize_contents_of_ron;
use crate::{
    CompanyInformation, Data, EncryptedEmailSettings, Error, ExpensedPeriods, Path, PathBuf,
    PaymentInformation, ProtoInvoiceInfo, Result, ServiceFees, Version,
};
use klirr_foundation::RonError;
pub use klirr_foundation::{data_dir, data_dir_create_if};
use log::debug;
use log::info;
use serde::Serialize;

const DATA_FILE_NAME_EMAIL_SETTINGS: &str = "email";
const DATA_FILE_NAME_VENDOR: &str = "vendor";
const DATA_FILE_NAME_CLIENT: &str = "client";
const DATA_FILE_NAME_PAYMENT: &str = "payment";
const DATA_FILE_NAME_SERVICE_FEES: &str = "service_fees";
const DATA_FILE_NAME_PROTO_INVOICE_INFO: &str = "invoice_info";
const DATA_FILE_NAME_EXPENSES: &str = "expenses";
const DATA_FILE_NAME_CACHED_RATES: &str = "cached_rates";
const DATA_FILE_NAME_VERSION: &str = "version";

pub(crate) fn save_to_disk<T: Serialize>(model: &T, path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    klirr_foundation::save_to_disk(model, path).map_err(map_ron_error)?;
    info!("✅ Successfully saved file at: {}", path.display());
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

pub fn save_data_with_base_path(data: Data, base_path: impl AsRef<Path>) -> Result<()> {
    let base_path = base_path.as_ref();
    save_to_disk(data.version(), version_path(base_path))?;
    save_to_disk(data.vendor(), vendor_path(base_path))?;
    save_to_disk(data.client(), client_path(base_path))?;
    save_to_disk(data.information(), proto_invoice_info_path(base_path))?;
    save_to_disk(data.payment_info(), payment_info_path(base_path))?;
    save_to_disk(data.service_fees(), service_fees_path(base_path))?;
    save_to_disk(data.expensed_periods(), expensed_periods_path(base_path))?;
    Ok(())
}

pub fn path_to_ron_file_with_base(base_path: impl AsRef<Path>, name: &str) -> PathBuf {
    klirr_foundation::path_to_ron_file_with_base(base_path, name)
}

pub fn load_data<T: DeserializeOwned>(base_path: impl AsRef<Path>, name: &str) -> Result<T> {
    deserialize_contents_of_ron(path_to_ron_file_with_base(base_path, name))
}

fn map_ron_error(error: RonError) -> Error {
    match error {
        RonError::FileNotFound { path, underlying } => Error::FileNotFound { path, underlying },
        RonError::FailedToSerialize {
            type_name,
            underlying,
        } => Error::FailedToRonSerializeData {
            type_name,
            underlying,
        },
        RonError::FailedToWriteDataToDisk { underlying } => {
            Error::FailedToWriteDataToDisk { underlying }
        }
        RonError::Deserialize { type_name, error } => Error::Deserialize { type_name, error },
    }
}

pub fn email_settings_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_EMAIL_SETTINGS)
}

pub fn cached_rates_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_CACHED_RATES)
}

pub fn version_path(base_path: impl AsRef<Path>) -> PathBuf {
    path_to_ron_file_with_base(base_path, DATA_FILE_NAME_VERSION)
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

pub fn proto_invoice_info(base_path: impl AsRef<Path>) -> Result<ProtoInvoiceInfo> {
    deserialize_contents_of_ron(proto_invoice_info_path(base_path))
}

pub fn expensed_periods(base_path: impl AsRef<Path>) -> Result<ExpensedPeriods> {
    deserialize_contents_of_ron(expensed_periods_path(base_path))
}

pub fn read_email_data_from_disk_with_base_path(
    base_path: impl AsRef<Path>,
) -> Result<EncryptedEmailSettings> {
    deserialize_contents_of_ron(email_settings_path(base_path))
}

pub fn version(base_path: impl AsRef<Path>) -> Result<Version> {
    let base_path = base_path.as_ref();
    let path = version_path(base_path);
    if !path.exists() && base_path.exists() {
        let current = Version::current();
        if current == Version::V1 {
            // For backwards compatibility with pre-versioned on-disk data:
            // treat missing version.ron as V1 and persist it.
            save_to_disk(&current, &path)?;
            info!("Automatically migrated data to {}", current);
            return Ok(current);
        }
        // If we are beyond V1, a missing version.ron is ambiguous and
        // requires explicit migration instructions instead of auto-upgrading.
        return Err(Error::data_version_mismatch(Version::V0, current));
    }
    deserialize_contents_of_ron(path)
}

fn validate_data_version(version: Version) -> Result<()> {
    let current = Version::current();
    if version != current {
        return Err(Error::data_version_mismatch(version, current));
    }
    Ok(())
}

pub fn read_data_from_disk_with_base_path(base_path: impl AsRef<Path>) -> Result<Data> {
    let base_path = base_path.as_ref();
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    debug!("☑️ Reading data from disk...");
    let version = version(base_path)?;
    validate_data_version(version)?;
    let client = client(base_path)?;
    let vendor = vendor(base_path)?;
    let payment_info = payment_info(base_path)?;
    let service_fees = service_fees(base_path)?;
    let proto_invoice_info = proto_invoice_info(base_path)?;
    let expensed_periods = expensed_periods(base_path)?;

    let input_data = Data::builder()
        .version(version)
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
    use crate::HasSample;
    use test_log::test;

    #[test]
    fn write_read_validate_data() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let data = Data::sample();
        save_data_with_base_path(data.clone(), tempdir.path()).unwrap();
        let loaded_data = read_data_from_disk_with_base_path(tempdir.path()).unwrap();
        assert_eq!(loaded_data, data, "Loaded data should match saved data");
    }

    #[test]
    fn save_data_writes_version_file() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_data_with_base_path(Data::sample(), tempdir.path()).unwrap();

        let loaded_version: Version = deserialize_contents_of_ron(version_path(tempdir.path()))
            .expect("Expected version.ron to deserialize");
        assert_eq!(loaded_version, Version::current());
    }

    #[test]
    fn read_data_auto_migrates_when_version_file_is_missing_in_existing_data_dir() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_data_with_base_path(Data::sample(), tempdir.path()).unwrap();
        std::fs::remove_file(version_path(tempdir.path())).expect("Expected version.ron to exist");

        let result = read_data_from_disk_with_base_path(tempdir.path())
            .expect("Expected read_data_from_disk_with_base_path to auto-migrate version.ron");
        assert_eq!(*result.version(), Version::current());

        let persisted_version: Version = deserialize_contents_of_ron(version_path(tempdir.path()))
            .expect("Expected version.ron to be recreated");
        assert_eq!(persisted_version, Version::current());
    }

    #[test]
    fn read_data_fails_when_version_does_not_match_current() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_data_with_base_path(Data::sample(), tempdir.path()).unwrap();
        save_to_disk(&Version::V0, version_path(tempdir.path())).unwrap();

        let result = read_data_from_disk_with_base_path(tempdir.path());
        assert!(matches!(
            result,
            Err(Error::DataVersionMismatch { found, current })
                if found == Version::V0 && current == Version::current()
        ));
    }
}
