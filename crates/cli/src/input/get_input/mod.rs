mod data_admin_input;
mod email_input;
mod expenses_input;
#[allow(clippy::module_inception)]
mod get_input;

pub use data_admin_input::{
    DataAdminInput, DataAdminInputCommand, EditDataInput, EditDataInputSelector, PeriodOffInput,
};
pub use email_input::{EditEmailInput, EditEmailInputSelector, EmailInput, EmailInputCommand};
pub use expenses_input::ExpensesInput;
pub use get_input::{CliArgs, Command, InvoiceInput};
