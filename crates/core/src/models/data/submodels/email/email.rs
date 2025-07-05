use crate::prelude::*;

/// An email message that can be sent using an SMTP server.
#[derive(Debug, Clone, TypedBuilder, Getters)]
pub struct Email {
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    public_recipients: IndexSet<EmailAddress>,
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    cc_recipients: IndexSet<EmailAddress>,
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    bcc_recipients: IndexSet<EmailAddress>,
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    subject: String,
    #[builder(setter(into, strip_option), default)]
    body: Option<String>,

    #[builder(setter(into, strip_option), default)]
    #[getset(get = "pub")]
    reply_to: Option<EmailAccount>,

    /// Paths to attachments.
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    attachments: IndexSet<Attachment>,
}

impl Email {
    pub fn body(&self) -> String {
        self.body.clone().unwrap_or_default()
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
}
