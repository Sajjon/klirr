use inquire::{Confirm, CustomType};

use crate::{EmailAddress, Error, Result, WithOptionalRefDefault, format_help_skippable};
use derive_more::Display;
use indexmap::IndexSet;
use log::warn;

#[derive(Display, Clone, Copy, Debug)]
pub enum EmailAddressRole {
    #[display("Reply-To")]
    ReplyTo,
    #[display("Sender")]
    Sender,
    #[display("Recipient")]
    Recipient,
    #[display("CC")]
    Cc,
    #[display("BCC")]
    Bcc,
}

pub fn ask_for_email_address_skippable(
    role: EmailAddressRole,
    default: Option<&EmailAddress>,
) -> Result<Option<EmailAddress>> {
    CustomType::<EmailAddress>::new(&format!("{}'s email address?", role))
        .with_help_message(&format_help_skippable(format!(
            "Email address for {}",
            role
        )))
        .with_optional_ref_default(default)
        .prompt_skippable()
        .map_err(|e| Error::InvalidEmailAddress {
            role: role.to_string(),
            underlying: e.to_string(),
        })
}

pub fn ask_for_email_address(
    role: EmailAddressRole,
    default: &EmailAddress,
) -> Result<EmailAddress> {
    CustomType::<EmailAddress>::new(&format!("{}'s email address?", role))
        .with_help_message(&format!("Email address for {}", role))
        .with_default(default.clone())
        .prompt()
        .map_err(|e| Error::InvalidEmailAddress {
            role: role.to_string(),
            underlying: e.to_string(),
        })
}

pub fn ask_for_many_email_addresses(
    role: EmailAddressRole,
    defaults: &IndexSet<EmailAddress>,
) -> Result<IndexSet<EmailAddress>> {
    let mut emails = IndexSet::new();
    loop {
        let Some(email) = ask_for_email_address_skippable(role, defaults.get_index(emails.len()))?
        else {
            break;
        };
        if emails.contains(&email) {
            warn!("Email address already exists, skipping");
            continue;
        }
        emails.insert(email);
        let another = Confirm::new(&format!("Add another {} email address?", role))
            .with_default(true)
            .prompt()
            .unwrap_or(true);
        if !another {
            break;
        }
    }
    Ok(emails)
}
