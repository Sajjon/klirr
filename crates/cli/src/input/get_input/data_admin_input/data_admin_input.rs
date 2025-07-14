use crate::prelude::*;
use clap::Args;
use clap::Subcommand;
use derive_more::Unwrap;

/// The CLI arguments for data management, such as initializing the data directory,
/// validating the data, or recording expenses or month off.
#[derive(Debug, Args, Getters, PartialEq, Builder)]
pub struct DataAdminInput {
    /// The command to run for data management, such as initializing the data directory,
    /// validating the data, or recording expenses or month off.
    #[command(subcommand)]
    #[getset(get = "pub")]
    command: DataAdminInputCommand,
}

/// The commands available for data management, such as initializing the data directory,
/// validating the data, or recording expenses or month off.
#[derive(Debug, Subcommand, Unwrap, PartialEq)]
pub enum DataAdminInputCommand {
    /// Prints the data in the data directory as a RON object.
    Dump,
    /// Initializes the data in the data directory, creating it if it does not exist.
    /// Such as information about you as a vendor and your client, payment information
    /// pricing etc
    Init,
    /// Validates the data in the data directory, checking if it is correctly formatted
    /// and if all required fields are present.
    Validate,
    /// Just like `Init` but will use the existing data, prefilling the values
    /// with the existing data as default values so that user can press Enter
    /// to accept the existing values as defaults.
    Edit(EditDataInput),
    /// Records a month off for the specified month, which is used to calculate the invoice.
    MonthOff(MonthOffInput),
    /// Records expenses for the specified month, used to create expenses invoices
    /// and affects invoice number calculation.
    Expenses(ExpensesInput),
}
