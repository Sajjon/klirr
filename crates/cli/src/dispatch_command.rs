use crate::prelude::*;
use klirr_render::prelude::render;

fn init_data(provide_data: impl FnOnce(Data) -> Result<Data>) -> Result<()> {
    init_data_at(data_dir_create_if(true), provide_data)
}

fn edit_data(provide_data: impl FnOnce(Data) -> Result<Data>) -> Result<()> {
    edit_data_at(data_dir(), provide_data)
}

fn validate_data() -> Result<()> {
    let base_path = data_dir();
    info!("Validating data directory at: {}", base_path.display());

    read_data_from_disk_with_base_path(base_path)
        .map(|_| ())
        .inspect(|_| {
            info!("✅ Data directory is valid");
        })
        .inspect_err(|e| {
            error!("❌ Data directory is invalid: {}", e);
        })
}

fn record_expenses(month: &YearAndMonth, expenses: &[Item]) -> Result<()> {
    record_expenses_with_base_path(month, expenses, data_dir())
}

fn record_month_off(month: &YearAndMonth) -> Result<()> {
    record_month_off_with_base_path(month, data_dir())
}

/// Curry a function that takes two arguments into a function that takes one argument and returns another function.
/// This is useful for partially applying functions in a functional programming style.
fn curry<T, U, R>(f: impl FnOnce(T, U) -> R, u: U) -> impl FnOnce(T) -> R {
    move |t| f(t, u)
}

pub fn run_data_command(command: &DataAdminInputCommand) -> Result<()> {
    match command {
        DataAdminInputCommand::Init => init_data(curry(ask_for_data, None)),
        DataAdminInputCommand::Validate => validate_data(),
        DataAdminInputCommand::Edit(input) => edit_data(curry(
            ask_for_data,
            Some(DataSelector::from(*input.selector())),
        )),
        DataAdminInputCommand::MonthOff(month_off_input) => {
            record_month_off(month_off_input.month())
        }
        DataAdminInputCommand::Expenses(expenses_input) => {
            record_expenses(expenses_input.month(), expenses_input.expenses())
        }
    }
}

pub fn render_sample() -> Result<NamedPdf> {
    let path = dirs_next::home_dir()
        .expect("Expected to be able to find HOME dir")
        .join("klirr_sample.pdf");
    create_pdf_with_data(
        Data::sample(),
        ValidInput::builder()
            .maybe_output_path(path)
            .month(YearAndMonth::last())
            .build(),
        render,
    )
}

fn run_invoice_command_with_base_path(
    input: InvoiceInput,
    data_path: impl AsRef<Path>,
) -> Result<NamedPdf> {
    let input = input.parsed()?;
    info!("🔮 Starting PDF creation, input: {}...", input);
    let pdf_location = create_pdf_with_data_base_path(data_path, input, render)?;
    save_pdf_location_to_tmp_file(pdf_location.saved_at().clone())?;
    Ok(pdf_location)
}

fn init_email_data_with(provide_data: impl FnOnce() -> Result<EmailSettings>) -> Result<()> {
    init_email_data_at(data_dir_create_if(true), provide_data)
}
pub fn init_email_data() -> Result<()> {
    init_email_data_with(ask_for_email)
}
pub fn validate_email_data() -> Result<()> {
    unimplemented!()
}
pub fn load_email_data_and_send_test_email() -> Result<()> {
    unimplemented!()
}
pub fn run_email_command(command: &EmailInputCommand) -> Result<()> {
    match command {
        EmailInputCommand::Init => init_email_data(),
        EmailInputCommand::Validate => validate_email_data(),
        EmailInputCommand::Test => load_email_data_and_send_test_email(),
    }
}

pub fn run_invoice_command(input: InvoiceInput) -> Result<NamedPdf> {
    run_invoice_command_with_base_path(input, data_dir())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::InvoiceInput;
    use test_log::test;

    #[test]
    fn test_run_invoice_command() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let tempfile = tempdir.path().join("out.pdf");
        save_data_with_base_path(Data::sample(), tempdir.path()).unwrap();
        let input = InvoiceInput::parse_from([
            "invoice",
            "--out",
            &format!("{}", tempfile.as_path().display()),
        ]);
        let result = run_invoice_command_with_base_path(input, tempdir.path());
        assert!(result.is_ok(), "Expected run to succeed, got: {:?}", result);
    }
}
