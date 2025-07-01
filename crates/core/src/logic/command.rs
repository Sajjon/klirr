use crate::prelude::*;
use serde::de::DeserializeOwned;

fn input_data_at(
    default_data: Data,
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce(Data) -> Result<Data>,
) -> Result<()> {
    let data = provide_data(default_data)?;
    save_data_with_base_path(data, write_path)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSelector {
    All,
    Vendor,
    Client,
    Information,
    PaymentInfo,
    ServiceFees,
    ExpensedMonths,
}

impl DataSelector {
    pub fn includes(&self, target: DataSelector) -> bool {
        match self {
            DataSelector::All => true,
            DataSelector::Vendor => matches!(target, DataSelector::Vendor),
            DataSelector::Client => matches!(target, DataSelector::Client),
            DataSelector::Information => matches!(target, DataSelector::Information),
            DataSelector::PaymentInfo => matches!(target, DataSelector::PaymentInfo),
            DataSelector::ServiceFees => matches!(target, DataSelector::ServiceFees),
            DataSelector::ExpensedMonths => matches!(target, DataSelector::ExpensedMonths),
        }
    }
}

pub fn edit_data_at(
    path: impl AsRef<Path>,
    provide_data: impl FnOnce(Data) -> Result<Data>,
) -> Result<()> {
    let path = path.as_ref();
    info!("Editing data at: {}", path.display());
    let existing = read_data_from_disk_with_base_path(path)?;
    input_data_at(existing, path, provide_data)?;
    info!("✅ Data edit done");
    Ok(())
}

pub fn init_data_at(
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce(Data) -> Result<Data>,
) -> Result<()> {
    let write_path = write_path.as_ref();
    info!("Initializing data directory at: {}", write_path.display());
    input_data_at(Data::sample(), write_path, provide_data)?;
    info!("✅ Data init done, you're ready: `{} invoice`", BINARY_NAME);
    Ok(())
}

fn mutate<D: Serialize + DeserializeOwned + Clone>(
    data_path: impl AsRef<Path>,
    data_file_name: &str,
    mutate: impl FnOnce(&mut D),
) -> Result<()> {
    let data_path = data_path.as_ref();
    let mut data = load_data::<D>(data_path, data_file_name)?.clone();
    mutate(&mut data);
    let path = path_to_ron_file_with_base(data_path, data_file_name);
    save_to_disk(&data, path)?;
    Ok(())
}

pub fn record_expenses_with_base_path(
    month: &YearAndMonth,
    expenses: &[Item],
    data_path: impl AsRef<Path>,
) -> Result<()> {
    info!("Recording #{} expenses for: {}", expenses.len(), month);
    mutate(
        data_path,
        DATA_FILE_NAME_EXPENSES,
        |data: &mut ExpensedMonths| {
            data.insert_expenses(month, expenses.to_vec());
        },
    )
    .inspect(|_| {
        info!("✅ Expenses recorded successfully");
    })
}

pub fn record_month_off_with_base_path(
    month: &YearAndMonth,
    data_path: impl AsRef<Path>,
) -> Result<()> {
    info!("Recording month off for: {}", month);
    mutate(
        data_path,
        DATA_FILE_NAME_PROTO_INVOICE_INFO,
        |data: &mut ProtoInvoiceInfo| {
            data.insert_month_off(*month);
        },
    )
    .inspect(|_| {
        info!("✅ Month off recorded successfully");
    })
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
    fn test_read_data_from_disk() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_data_with_base_path(Data::sample(), tempdir.path()).unwrap();
        let result = read_data_from_disk_with_base_path(tempdir.path());
        assert!(
            result.is_ok(),
            "Expected validation to succeed, got: {:?}",
            result
        );
    }

    #[test]
    fn test_init_data_directory_at() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let result = init_data_at(tempdir.path(), Ok);
        assert!(
            result.is_ok(),
            "Expected data directory initialization to succeed, got: {:?}",
            result
        );
    }

    #[test]
    fn test_record_month_off_with_base_path() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let month = YearAndMonth::may(2025);
        save_to_disk(
            &ProtoInvoiceInfo::sample(),
            path_to_ron_file_with_base(tempdir.path(), DATA_FILE_NAME_PROTO_INVOICE_INFO),
        )
        .unwrap();
        record_month_off_with_base_path(&month, tempdir.path()).unwrap();

        // Verify that the month was recorded correctly
        let data = proto_invoice_info(tempdir.path()).unwrap();
        assert!(data.months_off_record().contains(&month));
    }

    #[test]
    fn test_record_expenses_with_base_path() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_to_disk(
            &ExpensedMonths::sample(),
            path_to_ron_file_with_base(tempdir.path(), DATA_FILE_NAME_EXPENSES),
        )
        .unwrap();
        let month = YearAndMonth::may(2025);
        let expenses = vec![Item::sample_expense_breakfast()];

        record_expenses_with_base_path(&month, &expenses, tempdir.path()).unwrap();

        // Verify that the month was recorded correctly
        let data = expensed_months(tempdir.path()).unwrap();
        assert!(data.contains(&month));
    }
}
