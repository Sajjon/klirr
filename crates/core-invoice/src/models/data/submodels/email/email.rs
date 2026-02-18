use crate::{Attachment, DecryptedEmailSettings, EmailAccount, EmailAddress, HasSample, NamedPdf};
use bon::Builder;
use getset::Getters;
use indexmap::IndexSet;

/// An email message that can be sent using an SMTP server.
#[derive(Debug, Clone, Builder, Getters, PartialEq)]
pub struct Email {
    /// The public recipients of the email
    #[builder(default)]
    #[getset(get = "pub")]
    public_recipients: IndexSet<EmailAddress>,

    /// The carbon copy recipients of the email
    #[builder(default)]
    #[getset(get = "pub")]
    cc_recipients: IndexSet<EmailAddress>,

    /// The blind carbon copy recipients of the email.
    #[builder(default)]
    #[getset(get = "pub")]
    bcc_recipients: IndexSet<EmailAddress>,

    /// The subject of the email
    #[builder(default)]
    #[getset(get = "pub")]
    subject: String,

    /// The body of the email
    body: Option<String>,

    /// An optional reply to which overrides the reply-to-sender
    #[getset(get = "pub")]
    reply_to: Option<EmailAccount>,

    /// Paths to attachments.
    #[builder(default)]
    #[getset(get = "pub")]
    attachments: IndexSet<Attachment>,
}

impl Email {
    /// Returns the body of the email or empty if not set
    pub fn body(&self) -> String {
        self.body.clone().unwrap_or_default()
    }
}

impl From<(DecryptedEmailSettings, NamedPdf)> for Email {
    fn from((settings, pdf): (DecryptedEmailSettings, NamedPdf)) -> Self {
        let (subject, body) = settings.template().materialize(pdf.prepared_data());
        Email::builder()
            .subject(subject)
            .body(body)
            .maybe_reply_to(settings.reply_to().clone())
            .public_recipients(settings.recipients().clone())
            .cc_recipients(settings.cc_recipients().clone())
            .bcc_recipients(settings.bcc_recipients().clone())
            .attachments(IndexSet::from([Attachment::Pdf(pdf)]))
            .build()
    }
}

impl HasSample for Email {
    fn sample() -> Self {
        Self::builder()
            .public_recipients(IndexSet::from_iter(vec![EmailAddress::sample_bob()]))
            .cc_recipients(IndexSet::from_iter(vec![EmailAddress::sample_carol()]))
            .bcc_recipients(IndexSet::from_iter(vec![EmailAddress::sample_erin()]))
            .subject("Sample Email Subject".to_string())
            .body("This is a sample email body.".to_string())
            .attachments(IndexSet::from_iter(vec![Attachment::Pdf(
                NamedPdf::sample(),
            )]))
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .public_recipients(IndexSet::from_iter(vec![EmailAddress::sample_alice()]))
            .cc_recipients(IndexSet::from_iter(vec![EmailAddress::sample_dave()]))
            .subject("Another Sample Email Subject".to_string())
            .body("This is another sample email body.".to_string())
            .attachments(IndexSet::from_iter(vec![Attachment::Pdf(
                NamedPdf::sample_other(),
            )]))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use secrecy::SecretString;

    use super::*;
    use crate::HasSample;
    use crate::{Salt, SmtpServer, Template};
    use std::str::FromStr;

    type Sut = Email;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn reply_to() {
        let address = EmailAddress::from_str("my@reply.to").unwrap();
        let reply_to = EmailAccount::builder()
            .email(address)
            .name("Satoshi Nakamoto".to_owned())
            .build();
        let email_settings = DecryptedEmailSettings::builder()
            .reply_to(reply_to.clone())
            .smtp_app_password(SecretString::sample())
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
            .build();
        let email = Sut::from((email_settings, NamedPdf::sample()));
        assert_eq!(email.reply_to(), &Some(reply_to));
    }
}
