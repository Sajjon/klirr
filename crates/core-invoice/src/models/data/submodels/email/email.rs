use crate::prelude::*;

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
    use super::*;

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
}
