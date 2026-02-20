use crate::Select;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailSettingsSelector {
    /// All editable email settings.
    All,
    /// The SMTP app password value.
    AppPassword,
    /// The encryption password for encrypted secrets.
    EncryptionPassword,
    /// The email subject/body template.
    Template,
    /// The SMTP server host configuration.
    SmtpServer,
    /// Reply-to email address.
    ReplyTo,
    /// Sender account/address.
    Sender,
    /// Primary recipients.
    Recipients,
    /// CC recipients.
    CcRecipients,
    /// BCC recipients.
    BccRecipients,
}

impl EmailSettingsSelector {
    /// Returns whether selecting this field requires the encryption password.
    pub fn requires_encryption_password(&self) -> bool {
        use EmailSettingsSelector::*;
        match self {
            All | AppPassword | EncryptionPassword => true,
            Template | SmtpServer | ReplyTo | Sender | Recipients | CcRecipients
            | BccRecipients => false,
        }
    }
}

impl Select for EmailSettingsSelector {
    fn includes(&self, target: Self) -> bool {
        match self {
            EmailSettingsSelector::All => true,
            EmailSettingsSelector::AppPassword => {
                matches!(target, EmailSettingsSelector::AppPassword)
            }
            EmailSettingsSelector::EncryptionPassword => {
                matches!(target, EmailSettingsSelector::EncryptionPassword)
            }
            EmailSettingsSelector::Template => {
                matches!(target, EmailSettingsSelector::Template)
            }
            EmailSettingsSelector::SmtpServer => {
                matches!(target, EmailSettingsSelector::SmtpServer)
            }
            EmailSettingsSelector::ReplyTo => matches!(target, EmailSettingsSelector::ReplyTo),
            EmailSettingsSelector::Sender => matches!(target, EmailSettingsSelector::Sender),
            EmailSettingsSelector::Recipients => {
                matches!(target, EmailSettingsSelector::Recipients)
            }
            EmailSettingsSelector::CcRecipients => {
                matches!(target, EmailSettingsSelector::CcRecipients)
            }
            EmailSettingsSelector::BccRecipients => {
                matches!(target, EmailSettingsSelector::BccRecipients)
            }
        }
    }
}
