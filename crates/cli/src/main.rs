#![cfg_attr(not(test), forbid(unsafe_code))]

mod dispatch_command;
mod error;
mod init_logging;
mod input;
mod run;

pub(crate) use klirr_core_invoice::{
    BINARY_NAME, Cadence, CompanyInformation, Currency, Data, DataSelector, Date, Decimal,
    DecryptedEmailSettings, EmailAccount, EmailAddress, EmailSettingsSelector,
    EncryptedAppPassword, EncryptedEmailSettings, FooterText, Granularity, HasSample, HexColor,
    InvoiceNumber, InvoicedItems, IsPeriod, Item, Language, Month, MonthHalf, NamedPdf, Path,
    PathBuf, PaymentInformation, PaymentTerms, PeriodAnno, PostalAddress, ProtoInvoiceInfo,
    PurchaseOrder, Quantity, Rate, ResultExt, Salt, Select, ServiceFees, SmtpServer, StreetAddress,
    Template, TemplatePart, TimeOff, TimestampedInvoiceNumber, UnitPrice, ValidInput, Year,
    YearAndMonth, YearMonthAndFortnight, client_path, create_invoice_pdf_with_data, curry1, curry2,
    data_dir, data_dir_create_if, edit_data_at, edit_email_data_at, expensed_periods_path,
    init_data_at, init_email_data_at, load_email_data_and_send_test_email_at, payment_info_path,
    proto_invoice_info_path, read_data_from_disk_with_base_path, record_expenses_with_base_path,
    record_period_off_with_base_path, save_pdf_location_to_tmp_file,
    send_email_with_settings_for_pdf, service_fees_path, validate_email_data_at, vendor_path,
};

pub(crate) use crate::dispatch_command::{
    render_invoice_sample, render_invoice_sample_with_nonce, run_data_command, run_email_command,
    run_invoice_command, validate_email_data,
};
pub(crate) use crate::error::{
    CliError as Error, CliResult, EmailFromTuiError, InvoiceDataFromTuiError, Result,
};
pub(crate) use crate::init_logging::init_logging;
pub(crate) use crate::input::{
    CliArgs, Command, DataAdminInput, DataAdminInputCommand, EditDataInput, EditEmailInput,
    EmailAddressRole, EmailInput, EmailInputCommand, ExpensesInput, InvoiceInput, PeriodOffInput,
    TargetItems, TargetPeriod, TimeOffInput, TimeUnitInput, WithOptionalDefault,
    WithOptionalRefDefault, WithPossibleValues, ask_for_data, ask_for_email, ask_for_email_account,
    ask_for_email_account_skippable, ask_for_email_address, ask_for_email_address_skippable,
    ask_for_email_encryption_password_with_confirmation, ask_for_many_email_addresses,
    ask_for_password, ask_for_smtp_server, ask_for_template, build_company, build_invoice_info,
    build_payment_info, build_period, build_postal_address, build_service_fees,
    build_year_month_inner, config_render, format_help_skippable, get_email_encryption_password,
    select_or_default,
};
pub(crate) use crate::run::run;

/// Main function of the CLI program, runs the specified command according to the
/// args passed.
fn main() {
    use crate::{CliArgs, init_logging, run};
    use clap::Parser;
    init_logging();
    let input = CliArgs::parse();
    if run(input).is_err() {
        std::process::exit(1);
    }
}
