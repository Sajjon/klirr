mod build_company;
mod build_date;
mod build_email_account;
mod build_email_address;
mod build_email_template;
mod build_invoice_info;
mod build_password;
mod build_payment_info;
mod build_period;
mod build_postal_address;
mod build_service_fees;
mod build_smtp_server;
mod build_year_and_month;
mod select_or_default;

pub use build_company::*;
pub use build_email_account::*;
pub use build_email_address::*;
pub use build_email_template::*;
pub use build_invoice_info::*;
pub use build_password::*;
pub use build_payment_info::*;
pub use build_period::*;
pub use build_postal_address::*;
pub use build_service_fees::*;
pub use build_smtp_server::*;
pub use build_year_and_month::*;
pub use select_or_default::*;

use inquire::{
    set_global_render_config,
    ui::{RenderConfig, StyleSheet},
};
const HOW_TO_SKIP_INSTRUCTION: &str = "Skip with ESC";

pub fn format_help_skippable(help: impl Into<Option<String>>) -> String {
    help.into().map_or_else(
        || HOW_TO_SKIP_INSTRUCTION.to_owned(),
        |h| format!("{HOW_TO_SKIP_INSTRUCTION}: {h}"),
    )
}

pub fn config_render() {
    set_global_render_config(
        RenderConfig::default_colored().with_canceled_prompt_indicator(
            inquire::ui::Styled::new("Skipped")
                .with_style_sheet(StyleSheet::new().with_fg(inquire::ui::Color::LightBlue)),
        ),
    );
}
