use crate::prelude::*;

pub fn init_data_directory(provide_data: impl FnOnce() -> Result<Data>) -> Result<()> {
    init_data_directory_at(data_dir(), provide_data)
}

fn init_data_directory_at(
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce() -> Result<Data>,
) -> Result<()> {
    let data_dir = data_dir();
    info!("Initializing data directory at: {}", data_dir.display());
    let data = provide_data()?;
    info!("Data init successfully, saving to: {}", data_dir.display());
    save_data_with_base_path(data, write_path)?;
    info!("✅ Data init done, you're ready: `{} invoice`", BINARY_NAME);
    Ok(())
}

pub fn validate_data_directory() -> Result<()> {
    validate_data_directory_with_base_path(data_dir())
}

pub fn record_expenses(month: &YearAndMonth, expenses: &[Item]) -> Result<()> {
    record_expenses_with_base_path(month, expenses, data_dir())
}

pub fn record_month_off(month: &YearAndMonth) -> Result<()> {
    record_month_off_with_base_path(month, data_dir())
}

fn record_expenses_with_base_path(
    month: &YearAndMonth,
    expenses: &[Item],
    data_path: impl AsRef<Path>,
) -> Result<()> {
    info!("Recording #{} expenses for: {}", expenses.len(), month);
    let data_path = data_path.as_ref();
    let mut current = read_data_from_disk_with_base_path(data_path)?
        .expensed_months()
        .clone();
    current.insert_expenses(month, expenses.to_vec());
    save_to_disk(
        &current,
        path_to_ron_file_with_base(data_path, DATA_FILE_NAME_EXPENSES),
    )?;
    info!("✅ Expenses recorded successfully");
    Ok(())
}

fn record_month_off_with_base_path(
    month: &YearAndMonth,
    data_path: impl AsRef<Path>,
) -> Result<()> {
    info!("Recording month off for: {}", month);
    let data_path = data_path.as_ref();
    let mut current = read_data_from_disk_with_base_path(data_path)?
        .information()
        .clone();

    current.insert_month_off(*month);

    save_to_disk(
        &current,
        path_to_ron_file_with_base(data_path, DATA_FILE_NAME_PROTO_INVOICE_INFO),
    )?;
    info!("✅ Expenses recorded successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn save_to_disk_err_serialize() {
        use serde::{self, Serialize, Serializer};
        struct FailModel;

        impl Serialize for FailModel {
            fn serialize<S>(&self, _serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                Err(serde::ser::Error::custom(
                    "manual failure during serialization",
                ))
            }
        }

        let fail_model = FailModel;
        let result = save_to_disk(&fail_model, PathBuf::from("irrelevant"));
        assert!(result.is_err(), "Expected save to fail, got: {:?}", result);
    }

    #[test]
    fn save_to_disk_err_invalid_path() {
        let result = save_to_disk(
            &CompanyInformation::sample_client(),
            PathBuf::from("/invalid/path"),
        );
        assert!(result.is_err(), "Expected save to fail, got: {:?}", result);
    }

    #[test]
    fn test_validate_data_directory() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_data_with_base_path(Data::sample(), tempdir.path()).unwrap();
        let result = validate_data_directory_with_base_path(tempdir.path());
        assert!(
            result.is_ok(),
            "Expected validation to succeed, got: {:?}",
            result
        );
    }

    #[test]
    fn test_init_data_directory_at() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let result = init_data_directory_at(tempdir.path(), || Ok(Data::sample()));
        assert!(
            result.is_ok(),
            "Expected data directory initialization to succeed, got: {:?}",
            result
        );
    }
}
