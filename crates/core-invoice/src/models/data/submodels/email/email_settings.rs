use secrecy::SecretString;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::prelude::*;

pub type DecryptedEmailSettings = EmailSettings<SecretString>;
pub type EncryptedEmailSettings = EmailSettings<EncryptedAppPassword>;

/// Represents the settings for sending emails, including SMTP server details,
/// sender information, and recipient lists.
#[derive(
    derive_more::Debug, Clone, PartialEq, Eq, Builder, Getters, WithSetters, Serialize, Deserialize,
)]
pub struct EmailSettings<AppPassword: Zeroize> {
    /// The password for the SMTP server, typically an "App Password".
    #[getset(get = "pub")]
    #[debug("omitted")]
    smtp_app_password: AppPassword,

    /// Salt used to form the encryption key, together with the encryption password
    #[getset(get = "pub")]
    #[debug("omitted")]
    salt: Salt,

    /// The template for the email, containing subject and body formats.
    #[getset(get = "pub")]
    template: Template,

    /// The email address to reply to, if different from the sender, use None
    /// to indicate that the reply should go to the sender's email address.
    #[getset(get = "pub")]
    reply_to: Option<EmailAccount>,

    /// The SMTP server to use for sending the email.
    #[getset(get = "pub")]
    smtp_server: SmtpServer,

    /// The email account that will send the email.
    #[getset(get = "pub", set_with = "pub")]
    sender: EmailAccount,

    /// Public recipients of the email.
    #[getset(get = "pub")]
    recipients: IndexSet<EmailAddress>,

    // CC recipients of the email.
    #[getset(get = "pub")]
    cc_recipients: IndexSet<EmailAddress>,

    /// BCC recipients of the email (Blind Carbon Copy).
    #[getset(get = "pub")]
    bcc_recipients: IndexSet<EmailAddress>,
}

impl<AppPassword: Zeroize> Zeroize for EmailSettings<AppPassword> {
    fn zeroize(&mut self) {
        self.smtp_app_password.zeroize();
        self.salt.zeroize();
    }
}

impl<AppPassword: Zeroize> Drop for EmailSettings<AppPassword> {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl<AppPassword: Zeroize> ZeroizeOnDrop for EmailSettings<AppPassword> {}

impl DecryptedEmailSettings {
    pub fn compose(&self, pdf: &NamedPdf) -> (Email, EmailCredentials) {
        let email = Email::from((self.clone(), pdf.clone()));
        let credentials = EmailCredentials::from(self.clone());
        (email, credentials)
    }
}

impl<T: Zeroize + HasSample> HasSample for EmailSettings<T> {
    fn sample() -> Self {
        Self::builder()
            .smtp_app_password(T::sample())
            .salt(Salt::sample())
            .template(Template::default())
            .smtp_server(SmtpServer::default())
            .sender(EmailAccount::sample())
            .recipients(IndexSet::from([
                EmailAddress::sample_alice(),
                EmailAddress::sample_bob(),
            ]))
            .cc_recipients(IndexSet::from([EmailAddress::sample_carol()]))
            .bcc_recipients(IndexSet::from([
                EmailAddress::sample_dave(),
                EmailAddress::sample_erin(),
            ]))
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .smtp_app_password(T::sample_other())
            .salt(Salt::sample())
            .template(Template::default())
            .smtp_server(SmtpServer::default())
            .sender(EmailAccount::sample_other())
            .recipients(IndexSet::from([
                EmailAddress::sample_bob(),
                EmailAddress::sample_carol(),
            ]))
            .cc_recipients(IndexSet::from([EmailAddress::sample_dave()]))
            .bcc_recipients(IndexSet::from([
                EmailAddress::sample_erin(),
                EmailAddress::sample_alice(),
            ]))
            .build()
    }
}

impl EncryptedEmailSettings {
    fn derive_and_decrypt_smtp_app_password(
        &self,
        encryption_key: EncryptionKey,
    ) -> Result<DecryptedEmailSettings> {
        let decrypted = self.smtp_app_password.decrypt(encryption_key)?;
        Ok(DecryptedEmailSettings::builder()
            .smtp_app_password(decrypted)
            .maybe_reply_to(self.reply_to.clone())
            .smtp_server(self.smtp_server.clone())
            .sender(self.sender.clone())
            .recipients(self.recipients.clone())
            .cc_recipients(self.cc_recipients.clone())
            .bcc_recipients(self.bcc_recipients.clone())
            .template(self.template.clone())
            .salt(self.salt().clone())
            .build())
    }

    pub fn decrypt_smtp_app_password(
        &self,
        encryption_password: SecretString,
    ) -> Result<DecryptedEmailSettings> {
        let encryption_key = PbHkdfSha256::derive_key_from(encryption_password, self.salt());
        self.derive_and_decrypt_smtp_app_password(encryption_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = EncryptedEmailSettings;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
