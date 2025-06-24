use crate::prelude::*;
use invoice_typst_render::prelude::render;

fn save_to_disk<T: Serialize>(model: &T, path: impl AsRef<Path>) -> Result<()> {
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

fn init_data_directory(provide_data: impl FnOnce() -> Result<Data>) -> Result<()> {
    let data_dir = data_dir();
    info!("Initializing data directory at: {}", data_dir.display());
    let data = provide_data()?;
    info!(
        "Data initialized successfully, saving to disk in folder: {}",
        data_dir.display()
    );

    save_to_disk(data.vendor(), data_path_ron_file(DATA_FILE_NAME_VENDOR))?;
    save_to_disk(data.client(), data_path_ron_file(DATA_FILE_NAME_CLIENT))?;
    save_to_disk(
        data.information(),
        data_path_ron_file(DATA_FILE_NAME_PROTO_INVOICE_INFO),
    )?;
    save_to_disk(
        data.payment_info(),
        data_path_ron_file(DATA_FILE_NAME_PAYMENT),
    )?;
    save_to_disk(
        data.service_fees(),
        data_path_ron_file(DATA_FILE_NAME_SERVICE_FEES),
    )?;
    save_to_disk(
        data.expensed_months(),
        data_path_ron_file(DATA_FILE_NAME_EXPENSES),
    )?;

    info!(
        "✅ Data directory initialized successfully. You are now ready to create invoices! Try `klirr invoice` to get started. Or 'klirr --help' for more information."
    );

    Ok(())
}

fn validate_data_directory() -> Result<()> {
    let data_dir = data_dir();
    info!("Validating data directory at: {}", data_dir.display());
    read_data_from_disk()
        .map(|_| {
            info!("✅ Data directory is valid");
        })
        .inspect_err(|e| {
            error!("❌ Data directory is invalid: {}", e);
        })
}

fn record_month_off(input: &MonthOffInput) -> Result<()> {
    info!("Recording month off for: {}", input.month());
    todo!("implement me")
}

fn record_expenses(input: &ExpensesInput) -> Result<()> {
    info!(
        "Recording #{} expenses for: {}",
        input.expenses().len(),
        input.month()
    );
    let mut current = read_data_from_disk()?.expensed_months().clone();
    current.insert_expenses(input.month(), input.expenses().to_vec());
    save_to_disk(&current, data_path_ron_file(DATA_FILE_NAME_EXPENSES))?;
    info!("✅ Expenses recorded successfully");
    Ok(())
}

fn run_data_command(
    command: &DataAdminInputCommands,
    provide_data: impl FnOnce() -> Result<Data>,
) -> Result<()> {
    match command {
        DataAdminInputCommands::Init => init_data_directory(provide_data),
        DataAdminInputCommands::Validate => validate_data_directory(),
        DataAdminInputCommands::MonthOff(month_off_input) => record_month_off(month_off_input),
        DataAdminInputCommands::Expenses(expenses_input) => record_expenses(expenses_input),
    }
}

pub(super) fn run_data_cmd(input: DataAdminInput) -> Result<()> {
    run_data_command(input.command(), ask_for_data)
}

pub(super) fn run_invoice_cmd(input: InvoiceInput) -> Result<()> {
    let input = input.parsed()?;
    info!("🔮 Starting PDF creation, input: {}...", input);
    let pdf_location = create_pdf(input, render)?;
    save_pdf_location_to_tmp_file(pdf_location)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::InvoiceInput;

    #[test]
    fn test_run_invoice_cmd() {
        let tempfile = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        let input = InvoiceInput::parse_from([
            "invoice",
            "--out",
            &format!("{}", tempfile.path().display()),
        ]);
        let result = run_invoice_cmd(input);
        assert!(result.is_ok(), "Expected run to succeed, got: {:?}", result);
    }

    #[test]
    fn test_run_data_cmd_init() {
        let input = &DataAdminInputCommands::Init;
        let result = run_data_command(input, || Ok(Data::sample()));
        assert!(
            result.is_ok(),
            "Expected init to succeed, got: {:?}",
            result
        );
    }
}
