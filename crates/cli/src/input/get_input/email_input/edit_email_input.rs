use clap::{Args, ValueEnum};
use derive_more::Unwrap;

use crate::prelude::*;

#[derive(Debug, Args, Getters, PartialEq)]
pub struct EditEmailInput {
    #[arg(value_enum)]
    #[getset(get = "pub")]
    selector: EditEmailInputSelector,
}

#[derive(Clone, Copy, Debug, Subcommand, Unwrap, PartialEq, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub enum EditEmailInputSelector {
    All,
    AppPassword,
    EncryptionPassword,
    Template,
    Smtp,
    ReplyTo,
    Sender,
    Recipients,
    Cc,
    Bcc,
}

impl From<EditEmailInputSelector> for EmailSettingsSelector {
    fn from(selector: EditEmailInputSelector) -> Self {
        match selector {
            EditEmailInputSelector::All => EmailSettingsSelector::All,
            EditEmailInputSelector::AppPassword => EmailSettingsSelector::AppPassword,
            EditEmailInputSelector::EncryptionPassword => EmailSettingsSelector::EncryptionPassword,
            EditEmailInputSelector::Template => EmailSettingsSelector::Template,
            EditEmailInputSelector::Smtp => EmailSettingsSelector::SmtpServer,
            EditEmailInputSelector::ReplyTo => EmailSettingsSelector::ReplyTo,
            EditEmailInputSelector::Sender => EmailSettingsSelector::Sender,
            EditEmailInputSelector::Recipients => EmailSettingsSelector::Recipients,
            EditEmailInputSelector::Cc => EmailSettingsSelector::CcRecipients,
            EditEmailInputSelector::Bcc => EmailSettingsSelector::BccRecipients,
        }
    }
}
