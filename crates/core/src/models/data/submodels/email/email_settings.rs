use crate::prelude::*;

pub type DecryptedEmailSettings = EmailSettings<String>;
pub type EncryptedEmailSettings = EmailSettings<EncryptedAppPassword>;

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct EmailSettings<AppPassword> {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    smtp_app_password: AppPassword,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    reply_to: Option<EmailAccount>,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    smtp_server: SmtpServer,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    sender: EmailAccount,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    public_recipients: IndexSet<EmailAddress>,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    cc_recipients: IndexSet<EmailAddress>,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    bcc_recipients: IndexSet<EmailAddress>,
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
            .public_recipients(self.public_recipients.clone())
            .cc_recipients(self.cc_recipients.clone())
            .bcc_recipients(self.bcc_recipients.clone())
            .build())
    }

    pub fn decrypt_smtp_app_password(
        &self,
        encryption_password: String,
    ) -> Result<DecryptedEmailSettings> {
        let encryption_key = PbHkdfSha256::derive_key(encryption_password);
        self.derive_and_decrypt_smtp_app_password(encryption_key)
    }
}
