use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct EmailSettings {
    #[builder(setter(into))]
    encrypted_smtp_app_password: EncryptedAppPassword,

    #[getset(get = "pub")]
    #[builder(setter(into))]
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

impl EmailSettings {
    pub fn decrypt_smtp_app_password(&self, encryption_key: EncryptionKey) -> Result<String> {
        self.encrypted_smtp_app_password.decrypt(encryption_key)
    }
}
