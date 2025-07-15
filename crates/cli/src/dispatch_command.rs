use crate::prelude::*;
use klirr_render::prelude::render;
use secrecy::SecretString;

fn init_email_data(
    provide_data: impl FnOnce(EncryptedEmailSettings) -> Result<EncryptedEmailSettings>,
) -> Result<()> {
    init_email_data_at(data_dir(), provide_data)
}

fn init_data(
    provide_data: impl FnOnce(Data<PeriodAnno>) -> Result<Data<PeriodAnno>>,
) -> Result<()> {
    init_data_at(data_dir_create_if(true), provide_data)
}

fn edit_data(
    provide_data: impl FnOnce(Data<PeriodAnno>) -> Result<Data<PeriodAnno>>,
) -> Result<()> {
    edit_data_at(data_dir(), provide_data)
}

fn edit_email_data(
    provide_data: impl FnOnce(EncryptedEmailSettings) -> Result<EncryptedEmailSettings>,
) -> Result<()> {
    edit_email_data_at(data_dir(), provide_data)
}

fn dump_data() -> Result<()> {
    let base_path = data_dir();
    info!("Dumping data directory at: {}", base_path.display());

    read_data_from_disk_with_base_path(base_path)
        .inspect(|model| {
            let ron_str = ron::ser::to_string_pretty(model, ron::ser::PrettyConfig::default())
                .expect("Failed to serialize data to RON");
            info!("✅ Data: {ron_str}");
        })
        .inspect_err(|e| {
            fn load_contents<F>(get_path: F) -> String where F: FnOnce(&Path) -> PathBuf {
                let path = get_path(&data_dir());
                std::fs::read_to_string(&path).unwrap_or_else(|_| {
                    panic!("Failed to read file at: {}", path.display())
                })
            }
            let information = load_contents(|path| proto_invoice_info_path(path));
            let vendor = load_contents(|path| vendor_path(path));
            let client = load_contents(|path| client_path(path));
            let payment_info = load_contents(|path| payment_info_path(path));
            let service_fees = load_contents(|path| service_fees_path(path));
            let expensed_periods = load_contents(|path| expensed_periods_path(path));
            let str = format!("information: {information}\nvendor: {vendor}\nclient: {client}\npayment_info: {payment_info}\nservice_fees: {service_fees}\nexpensed_periods: {expensed_periods}\n");
            error!("❌ Data directory is invalid: {}, is:\n\n{}", e, str);
        })
        .map_to_void()
}

fn validate_data() -> Result<()> {
    let base_path = data_dir();
    info!("Validating data directory at: {}", base_path.display());

    read_data_from_disk_with_base_path(base_path)
        .map_to_void()
        .inspect(|_| {
            info!("✅ Data directory is valid");
        })
        .inspect_err(|e| {
            error!("❌ Data directory is invalid: {}", e);
        })
}

fn record_expenses(period: &PeriodAnno, expenses: &[Item]) -> Result<()> {
    record_expenses_with_base_path(period, expenses, data_dir())
}

fn record_period_off(period: &PeriodAnno) -> Result<()> {
    record_period_off_with_base_path(period, data_dir())
}

pub fn run_data_command(command: &DataAdminInputCommand) -> Result<()> {
    match command {
        DataAdminInputCommand::Init => init_data(curry2(ask_for_data, None)),
        DataAdminInputCommand::Dump => dump_data(),
        DataAdminInputCommand::Validate => validate_data(),
        DataAdminInputCommand::Edit(input) => edit_data(curry2(
            ask_for_data,
            Some(DataSelector::from(*input.selector())),
        )),
        DataAdminInputCommand::PeriodOff(period_off_input) => {
            record_period_off(period_off_input.period())
        }
        DataAdminInputCommand::Expenses(expenses_input) => {
            record_expenses(expenses_input.period(), expenses_input.expenses())
        }
    }
}

pub fn render_sample() -> Result<NamedPdf> {
    render_sample_with_nonce(false)
}

/// The nonce is used to ensure that the PDF is unique each time it is rendered.
/// This is useful for testing purposes, to avoid email spamming protection mechanisms.
/// It is not meant to be used in production, where in fact we WANT the PDF to
/// be identical each time it is rendered.
pub fn render_sample_with_nonce(use_nonce: bool) -> Result<NamedPdf> {
    let path = dirs_next::home_dir()
        .expect("Expected to be able to find HOME dir")
        .join("klirr_sample.pdf");
    let mut data = Data::<YearAndMonth>::sample();
    if use_nonce {
        let vat = format!("VAT{} {}", rand::random::<u64>(), rand::random::<u64>());
        data = data
            .clone()
            .with_client(data.client().clone().with_vat_number(vat));
    }
    create_pdf_with_data(
        data,
        ValidInput::builder()
            .maybe_output_path(path)
            .period(YearMonthAndFortnight::last())
            .build(),
        render,
    )
}

fn run_invoice_command_with_base_path(
    input: InvoiceInput,
    data_path: impl AsRef<Path>,
) -> Result<NamedPdf> {
    let input = input.parsed()?;
    info!("🔮 Starting PDF creation, input: {:?}...", input);
    let email_settings = input.email().clone();
    let named_pdf = create_pdf_with_data_base_path(data_path, input, render)?;
    save_pdf_location_to_tmp_file(named_pdf.saved_at().clone())?;
    if let Some(email_settings) = email_settings {
        send_email_with_settings_for_pdf(&named_pdf, &email_settings)?
    }
    Ok(named_pdf)
}

fn validate_email_data_with(
    get_email_password: impl FnOnce() -> Result<SecretString>,
) -> Result<DecryptedEmailSettings> {
    validate_email_data_at(data_dir(), get_email_password)
}
pub fn validate_email_data() -> Result<DecryptedEmailSettings> {
    validate_email_data_with(get_email_encryption_password)
}

fn load_email_data_and_send_test_email_with(
    get_email_password: impl FnOnce() -> Result<SecretString>,
    render_sample: impl FnOnce() -> Result<NamedPdf>,
) -> Result<()> {
    load_email_data_and_send_test_email_at(data_dir(), get_email_password, render_sample)
}
pub fn load_email_data_and_send_test_email(
    render_sample: impl FnOnce() -> Result<NamedPdf>,
) -> Result<()> {
    load_email_data_and_send_test_email_with(get_email_encryption_password, render_sample)
}

pub fn run_email_command(
    command: &EmailInputCommand,
    render_sample: impl FnOnce() -> Result<NamedPdf>,
) -> Result<()> {
    match command {
        EmailInputCommand::Edit(input) => edit_email_data(curry2(
            ask_for_email,
            Some(EmailSettingsSelector::from(*input.selector())),
        )),
        EmailInputCommand::Init => init_email_data(curry2(ask_for_email, None)),
        EmailInputCommand::Validate => validate_email_data().map_to_void(),
        EmailInputCommand::Test => load_email_data_and_send_test_email(render_sample),
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
        save_data_with_base_path(Data::<YearAndMonth>::sample(), tempdir.path()).unwrap();
        let input = InvoiceInput::parse_from([
            "invoice",
            "--out",
            &format!("{}", tempfile.as_path().display()),
        ]);
        let result = run_invoice_command_with_base_path(input, tempdir.path());
        assert!(result.is_ok(), "Expected run to succeed, got: {:?}", result);
    }
}
