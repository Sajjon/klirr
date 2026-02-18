mod get_input;
mod target_items;
mod target_period;
mod time_off_input;
mod time_off_unit_input;
mod tui;

#[allow(unused_imports)]
pub use get_input::{
    CliArgs, Command, DataAdminInput, DataAdminInputCommand, EditDataInput, EditDataInputSelector,
    EditEmailInput, EditEmailInputSelector, EmailInput, EmailInputCommand, ExpensesInput,
    InvoiceInput, PeriodOffInput,
};
pub use target_items::TargetItems;
pub use target_period::TargetPeriod;
pub use time_off_input::TimeOffInput;
pub use time_off_unit_input::TimeUnitInput;
pub use tui::{
    EmailAddressRole, WithOptionalDefault, WithOptionalRefDefault, WithPossibleValues,
    ask_for_data, ask_for_email, ask_for_email_account, ask_for_email_account_skippable,
    ask_for_email_address, ask_for_email_address_skippable,
    ask_for_email_encryption_password_with_confirmation, ask_for_many_email_addresses,
    ask_for_password, ask_for_smtp_server, ask_for_template, build_company, build_invoice_info,
    build_payment_info, build_period, build_postal_address, build_service_fees,
    build_year_month_inner, config_render, format_help_skippable, get_email_encryption_password,
    select_or_default,
};
