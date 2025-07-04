use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::Credentials,
};

use crate::prelude::*;

impl From<Attachment> for SinglePart {
    fn from(attachment: Attachment) -> Self {
        match attachment {
            Attachment::Pdf(named_pdf) => named_pdf.into(),
        }
    }
}
impl From<NamedPdf> for SinglePart {
    fn from(named_pdf: NamedPdf) -> Self {
        lettre::message::Attachment::new(named_pdf.name().clone())
            .body(named_pdf.pdf().as_ref().clone(), ContentType::pdf())
    }
}

#[derive(Debug, Clone, TypedBuilder, Getters)]
pub struct EmailWithSender {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    email: Email,
    #[builder(setter(into))]
    #[getset(get = "pub")]
    sender: EmailAccount,
}

trait ApplicationPdf: Sized {
    fn pdf() -> Self;
}

impl ApplicationPdf for ContentType {
    fn pdf() -> Self {
        ContentType::parse("application/pdf").unwrap()
    }
}

impl From<EmailAddress> for lettre::Address {
    fn from(address: EmailAddress) -> Self {
        (*address).clone()
    }
}

impl TryFrom<EmailWithSender> for Message {
    type Error = crate::Error;
    fn try_from(email_with_sender: EmailWithSender) -> Result<Self> {
        let sender = email_with_sender.sender();
        let email = email_with_sender.email();
        let mut builder = Message::builder()
            .from(Mailbox::new(
                Some(sender.name().clone()),
                sender.email().clone().into(),
            ))
            .subject(email.subject().clone());

        if let Some(reply_to) = email.reply_to() {
            builder = builder.reply_to(Mailbox::new(
                Some(reply_to.name().clone()),
                reply_to.email().clone().into(),
            ));
        }

        for recipient in email.public_recipients() {
            builder = builder.to(Mailbox::new(None, recipient.clone().into()));
        }

        for recipient in email.cc_recipients() {
            builder = builder.cc(Mailbox::new(None, recipient.clone().into()));
        }

        for recipient in email.bcc_recipients() {
            builder = builder.bcc(Mailbox::new(None, recipient.clone().into()));
        }

        let attachments = email.attachments().clone();
        if attachments.is_empty() {
            builder.body(email.body().clone())
        } else {
            let mut multipart = MultiPart::mixed()
                .singlepart(SinglePart::plain(email.body().clone()))
                // Insert a space between the body and the attachments
                .singlepart(SinglePart::plain("\n".to_owned()));

            for attachment in attachments {
                multipart = multipart.singlepart(attachment.into());
            }

            builder.multipart(multipart)
        }
        .map_err(|e| crate::Error::CreateEmailError {
            underlying: format!("{:?}", e),
        })
    }
}

impl From<EmailCredentials> for Credentials {
    fn from(credentials: EmailCredentials) -> Self {
        Credentials::new(
            credentials.account().email().user().to_owned(),
            credentials.password().clone(),
        )
    }
}
pub fn send_email(email: Email, credentials: EmailCredentials) -> Result<()> {
    let email_with_sender = EmailWithSender::builder()
        .email(email)
        .sender(credentials.account().clone())
        .build();
    let email = Message::try_from(email_with_sender)?;

    let smtp_server = credentials.smtp_server().clone();
    let creds = Credentials::from(credentials);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(smtp_server.as_ref())
        .map_err(|e| crate::Error::CreateSmtpTransportError {
            underlying: format!("Failed to create SMTP transport: {:?}", e),
        })?
        .credentials(creds)
        .build();

    // Send the email
    let response = mailer
        .send(&email)
        .map_err(|e| crate::Error::SendEmailError {
            underlying: format!("Failed to send email: {:?}", e),
        })?;

    if !response.is_positive() {
        warn!("Email sent, but response was negative: {:?}", response);
    }
    Ok(())
}
