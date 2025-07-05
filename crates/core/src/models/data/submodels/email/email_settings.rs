use secrecy::SecretString;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::prelude::*;

pub type DecryptedEmailSettings = EmailSettings<String>;
pub type EncryptedEmailSettings = EmailSettings<EncryptedAppPassword>;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    TypedBuilder,
    Getters,
    Serialize,
    Deserialize,
    ZeroizeOnDrop,
    Zeroize,
)]
pub struct EmailSettings<AppPassword: Zeroize> {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    smtp_app_password: AppPassword,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    proto_email: ProtoEmail,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    reply_to: Option<EmailAccount>,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    smtp_server: SmtpServer,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    sender: EmailAccount,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    public_recipients: IndexSet<EmailAddress>,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
    cc_recipients: IndexSet<EmailAddress>,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[zeroize(skip)]
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
            .proto_email(self.proto_email.clone())
            .build())
    }

    pub fn decrypt_smtp_app_password(
        &self,
        encryption_password: SecretString,
    ) -> Result<DecryptedEmailSettings> {
        let encryption_key = PbHkdfSha256::derive_key_from(encryption_password);
        self.derive_and_decrypt_smtp_app_password(encryption_key)
    }
}
