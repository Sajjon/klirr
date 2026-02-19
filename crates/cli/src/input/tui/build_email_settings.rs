use crate::{
    EmailAddressRole, EmailFromTuiError, EmailSettingsSelector, EncryptedAppPassword,
    EncryptedEmailSettings, Result, Salt, ask_for_email_account, ask_for_email_account_skippable,
    ask_for_email_encryption_password_with_confirmation, ask_for_many_email_addresses,
    ask_for_password, ask_for_smtp_server, ask_for_template, config_render, select_or_default,
};
use log::info;

pub fn ask_for_email(
    default: EncryptedEmailSettings,
    data_selector: Option<EmailSettingsSelector>,
) -> Result<EncryptedEmailSettings> {
    config_render();

    let is_editing_but_skip_secrets = data_selector
        .as_ref()
        .map(|s| !s.requires_encryption_password())
        .unwrap_or(false);

    let (salt, app_password_encrypted) = if is_editing_but_skip_secrets {
        // not edit secrets
        (default.salt().clone(), default.smtp_app_password().clone())
    } else {
        // is init or edit secrets
        let app_password_plaintext = ask_for_password(
            true,
            "SMTP App Password",
            "Used to authenticate sender account",
        )?;
        // Generate cryptographical salt
        let salt = Salt::generate();
        let encryption_password = ask_for_email_encryption_password_with_confirmation(true)?;
        let encryption_key = EncryptedAppPassword::new_by_deriving_and_encrypting(
            app_password_plaintext,
            encryption_password,
            &salt,
        );
        (salt, encryption_key)
    };

    let smtp_server = select_or_default(
        data_selector,
        EmailSettingsSelector::SmtpServer,
        default.smtp_server(),
        ask_for_smtp_server,
    )?;

    let sender = select_or_default(
        data_selector,
        EmailSettingsSelector::Sender,
        default.sender(),
        |d| ask_for_email_account(EmailAddressRole::Sender, d),
    )?;

    let template = select_or_default(
        data_selector,
        EmailSettingsSelector::Template,
        default.template(),
        ask_for_template,
    )?;

    let reply_to = select_or_default(
        data_selector,
        EmailSettingsSelector::ReplyTo,
        default.reply_to(),
        |d| ask_for_email_account_skippable(EmailAddressRole::ReplyTo, d.as_ref()),
    )?;

    let recipients = select_or_default(
        data_selector,
        EmailSettingsSelector::Recipients,
        default.recipients(),
        |d| ask_for_many_email_addresses(EmailAddressRole::Recipient, d),
    )?;

    if recipients.is_empty() {
        return Err(EmailFromTuiError::RecipientAddressesCannotBeEmpty.into());
    }

    let cc_recipients = select_or_default(
        data_selector,
        EmailSettingsSelector::CcRecipients,
        default.cc_recipients(),
        |d| ask_for_many_email_addresses(EmailAddressRole::Cc, d),
    )?;

    let bcc_recipients = select_or_default(
        data_selector,
        EmailSettingsSelector::BccRecipients,
        default.bcc_recipients(),
        |d| ask_for_many_email_addresses(EmailAddressRole::Bcc, d),
    )?;

    let email_settings = EncryptedEmailSettings::builder()
        .sender(sender)
        .smtp_server(smtp_server)
        .smtp_app_password(app_password_encrypted)
        .maybe_reply_to(reply_to)
        .recipients(recipients.clone())
        .bcc_recipients(bcc_recipients)
        .cc_recipients(cc_recipients)
        .template(template)
        .salt(salt)
        .build();

    info!("Email settings initialized: {:?}", email_settings);

    Ok(email_settings)
}
