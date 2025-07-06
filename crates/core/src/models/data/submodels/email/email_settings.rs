use secrecy::SecretString;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::prelude::*;

pub type DecryptedEmailSettings = EmailSettings<SecretString>;
pub type EncryptedEmailSettings = EmailSettings<EncryptedAppPassword>;

/// Represents the settings for sending emails, including SMTP server details,
/// sender information, and recipient lists.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    TypedBuilder,
    Getters,
    WithSetters,
    Serialize,
    Deserialize,
    ZeroizeOnDrop,
    Zeroize,
)]
pub struct EmailSettings<AppPassword: Zeroize> {
    /// The password for the SMTP server, typically an "App Password".
    #[builder(setter(into))]
    #[getset(get = "pub")]
    smtp_app_password: AppPassword,

    /// Salt used to form the encryption key, together with the encryption password
    #[builder(setter(into))]
    #[getset(get = "pub")]
    salt: Salt,

    /// The template for the email, containing subject and body formats.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    template: Template,

    /// The email address to reply to, if different from the sender, use None
    /// to indicate that the reply should go to the sender's email address.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    reply_to: Option<EmailAccount>,

    /// The SMTP server to use for sending the email.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    smtp_server: SmtpServer,

    /// The email account that will send the email.
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    #[zeroize(skip)]
    sender: EmailAccount,

    /// Public recipients of the email.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    recipients: IndexSet<EmailAddress>,

    // CC recipients of the email.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    cc_recipients: IndexSet<EmailAddress>,

    /// BCC recipients of the email (Blind Carbon Copy).
    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    bcc_recipients: IndexSet<EmailAddress>,
}

impl<T: Zeroize + HasSample> HasSample for EmailSettings<T> {
    fn sample() -> Self {
        Self::builder()
            .smtp_app_password(T::sample())
            .salt(Salt::sample())
            .template(Template::default())
            .reply_to(None)
            .smtp_server(SmtpServer::default())
            .sender(EmailAccount::sample())
            .recipients([EmailAddress::sample_alice(), EmailAddress::sample_bob()])
            .cc_recipients([EmailAddress::sample_carol()])
            .bcc_recipients([EmailAddress::sample_dave(), EmailAddress::sample_erin()])
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
            .reply_to(self.reply_to.clone())
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
