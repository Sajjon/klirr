mod build_data;
mod build_email_settings;
mod helpers;
mod inquire_extensions;

pub use build_data::ask_for_data;
pub use build_email_settings::ask_for_email;
pub use helpers::{
    EmailAddressRole, ask_for_email_account, ask_for_email_account_skippable,
    ask_for_email_address, ask_for_email_address_skippable,
    ask_for_email_encryption_password_with_confirmation, ask_for_many_email_addresses,
    ask_for_password, ask_for_smtp_server, ask_for_template, build_company, build_invoice_info,
    build_payment_info, build_period, build_postal_address, build_service_fees,
    build_year_month_inner, config_render, format_help_skippable, get_email_encryption_password,
    select_or_default,
};
pub use inquire_extensions::{WithOptionalDefault, WithOptionalRefDefault, WithPossibleValues};
