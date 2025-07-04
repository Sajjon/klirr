use crate::prelude::*;

/// Credentials for an email account, typically used for sending emails via SMTP.
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedBuilder, Getters)]
pub struct EmailCredentials {
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    smtp_server: SmtpServer,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    account: EmailAccount,

    /// The password for the email account, typically an "App Password".
    ///
    /// See [info here](https://support.google.com/mail/answer/185833?hl=en)
    ///
    /// Create app passwordds for [your Google Account here](https://myaccount.google.com/apppasswords)
    #[builder(setter(into))]
    #[getset(get = "pub")]
    password: String,
}
