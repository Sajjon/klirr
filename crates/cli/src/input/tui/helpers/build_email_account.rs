use inquire::Text;

use crate::{
    EmailAccount, EmailAddressRole, EmailFromTuiError, Result, WithOptionalRefDefault,
    ask_for_email_address, ask_for_email_address_skippable, format_help_skippable,
};

pub fn ask_for_email_account(
    role: EmailAddressRole,
    default: &EmailAccount,
) -> Result<EmailAccount> {
    let name = Text::new(&format!("Email account {} name?", role))
        .with_help_message(&format!("Will show up as the {} name", role))
        .with_default(default.name())
        .prompt()
        .map_err(EmailFromTuiError::invalid_name_for_email_for_role(role))
        .map_err(crate::Error::from)?;
    let email = ask_for_email_address(role, default.email())?;
    Ok(EmailAccount::builder().name(name).email(email).build())
}

pub fn ask_for_email_account_skippable(
    role: EmailAddressRole,
    default: Option<&EmailAccount>,
) -> Result<Option<EmailAccount>> {
    let name = Text::new(&format!("Email account {} name?", role))
        .with_help_message(&format_help_skippable(format!(
            "Will show up as the {} name",
            role
        )))
        .with_optional_ref_default(default.as_ref().map(|d| d.name()))
        .prompt_skippable()
        .map_err(EmailFromTuiError::invalid_name_for_email_for_role(role))
        .map_err(crate::Error::from)?;
    let Some(name) = name else { return Ok(None) };
    let Some(email) = ask_for_email_address_skippable(role, default.as_ref().map(|d| d.email()))?
    else {
        return Ok(None);
    };
    Ok(Some(
        EmailAccount::builder().name(name).email(email).build(),
    ))
}
