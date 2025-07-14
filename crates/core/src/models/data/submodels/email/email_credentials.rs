use secrecy::{ExposeSecret, SecretString};

use crate::prelude::*;

/// Credentials for an email account, typically used for sending emails via SMTP.
#[derive(Debug, Clone, Builder, Getters)]
pub struct EmailCredentials {
    #[builder(default)]
    #[getset(get = "pub")]
    smtp_server: SmtpServer,

    #[getset(get = "pub")]
    account: EmailAccount,

    /// The password for the email account, typically an "App Password".
    ///
    /// See [info here](https://support.google.com/mail/answer/185833?hl=en)
    ///
    /// Create app passwordds for [your Google Account here](https://myaccount.google.com/apppasswords)
    #[getset(get = "pub")]
    password: SecretString,
}

impl From<DecryptedEmailSettings> for EmailCredentials {
    fn from(settings: DecryptedEmailSettings) -> Self {
        EmailCredentials::builder()
            .account(
                EmailAccount::builder()
                    .name(settings.sender().name().clone())
                    .email(settings.sender().email().clone())
                    .build(),
            )
            .password(settings.smtp_app_password().clone())
            .smtp_server(settings.smtp_server().clone())
            .build()
    }
}

impl PartialEq for EmailCredentials {
    fn eq(&self, other: &Self) -> bool {
        self.smtp_server == other.smtp_server
            && self.account == other.account
            && self.password.expose_secret() == other.password.expose_secret()
    }
}

impl HasSample for EmailCredentials {
    fn sample() -> Self {
        Self::builder()
            .smtp_server(SmtpServer::default())
            .account(EmailAccount::sample_alice())
            .password("open sesame".into())
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .smtp_server(SmtpServer::default())
            .account(EmailAccount::sample_bob())
            .password("super_secret".into())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = EmailCredentials;

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
