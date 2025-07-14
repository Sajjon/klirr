use clap::Args;
use derive_more::Unwrap;

use crate::prelude::*;

#[derive(Debug, Args, Getters, PartialEq)]
pub struct EmailInput {
    #[command(subcommand)]
    #[getset(get = "pub")]
    command: EmailInputCommand,
}

#[derive(Debug, Subcommand, Unwrap, PartialEq)]
pub enum EmailInputCommand {
    /// Initializes the data related to sending emails in the data directory,
    Init,
    /// Validates the data related to sending emails in the data directory,
    Validate,
    Edit(EditEmailInput),
    /// Sends an email with a sample invoice as PDF attachment using the data
    /// in the data directory, which includes email account, SMTP server and
    /// recipient information.
    Test,
}
