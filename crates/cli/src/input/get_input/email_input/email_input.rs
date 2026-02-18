use clap::Args;
use derive_more::Unwrap;

use crate::EditEmailInput;
use clap::Subcommand;
use getset::Getters;

/// Email command input, see [`EmailInputCommand`]
#[derive(Debug, Args, Getters, PartialEq)]
pub struct EmailInput {
    /// Email command input, see [`EmailInputCommand`]
    #[command(subcommand)]
    #[getset(get = "pub")]
    command: EmailInputCommand,
}

/// Email subcommands either init email config, validate, edit or test.
#[derive(Debug, Subcommand, Unwrap, PartialEq)]
pub enum EmailInputCommand {
    /// Initializes the data related to sending emails in the data directory,
    Init,

    /// Validates the data related to sending emails in the data directory,
    Validate,

    /// Edit email input
    Edit(EditEmailInput),

    /// Sends an email with a sample invoice as PDF attachment using the data
    /// in the data directory, which includes email account, SMTP server and
    /// recipient information.
    Test,
}
