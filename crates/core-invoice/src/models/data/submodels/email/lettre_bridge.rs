use lettre::{
    Message,
    message::{Mailbox, MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use secrecy::ExposeSecret;

use crate::{Attachment, Email, EmailAccount, EmailAddress, EmailCredentials, Result};
use bon::Builder;
use getset::Getters;

/// An ephemeral helper struct to hold an email and its sender.
/// This is used to convert the email into a `lettre::Message`.
#[derive(Debug, Clone, Builder, Getters)]
pub struct EmailWithSender {
    #[getset(get = "pub")]
    email: Email,
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

impl From<Attachment> for SinglePart {
    fn from(attachment: Attachment) -> Self {
        match attachment {
            Attachment::Pdf(named_pdf) => {
                lettre::message::Attachment::new(named_pdf.name().clone())
                    .body(named_pdf.pdf().as_ref().clone(), ContentType::pdf())
            }
        }
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
            credentials.password().expose_secret().to_owned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use crate::{NamedPdf, PathBuf, PreparedData};
    use base64::{Engine, prelude::BASE64_STANDARD};
    use indexmap::IndexSet;
    use klirr_foundation::Pdf;
    use lettre::transport::smtp::authentication::Mechanism;
    use test_log::test;

    #[test]
    fn test_singlepart_from_attachment() {
        let named_pdf = NamedPdf::builder()
            .name("test.pdf".to_string())
            .pdf(Pdf(vec![0xde, 0xad, 0xbe, 0xef])) // Sample PDF data
            .prepared_data(PreparedData::sample())
            .saved_at(PathBuf::from("/tmp/test.pdf"))
            .build();
        let attachment: Attachment = named_pdf.into();
        let single_part: SinglePart = attachment.into();

        assert_eq!(hex::encode(single_part.formatted()), "436f6e74656e742d446973706f736974696f6e3a206174746163686d656e743b2066696c656e616d653d22746573742e706466220d0a436f6e74656e742d547970653a206170706c69636174696f6e2f7064660d0a436f6e74656e742d5472616e736665722d456e636f64696e673a206261736536340d0a0d0a3371322b37773d3d0d0a".to_owned());
    }

    #[test]
    fn test_content_type_pdf() {
        let content_type = ContentType::pdf();
        assert!(format!("{:?}", content_type).contains("application/pdf"));
    }

    #[test]
    fn test_credentials_from() {
        // convert into lettre::Credentials from EmailCredentials
        let email_credentials = EmailCredentials::sample();
        let credentials: Credentials = email_credentials.into();

        let mechanism = Mechanism::Login;

        assert_eq!(
            mechanism.response(&credentials, Some("Username")).unwrap(),
            "alice"
        );
        assert_eq!(
            mechanism.response(&credentials, Some("Password")).unwrap(),
            "open sesame"
        );
        assert!(mechanism.response(&credentials, None).is_err());
    }

    #[test]
    fn test_email_from() {
        let email = Email::sample();
        let sender = EmailAccount::sample_alice();
        let email_with_sender = EmailWithSender::builder()
            .email(email.clone())
            .sender(sender.clone())
            .build();
        let message = Message::try_from(email_with_sender).unwrap();
        let encoded = message.formatted();
        let decoded = String::from_utf8(encoded).unwrap();
        assert!(decoded.contains(&sender.email().to_string()));
        assert!(decoded.contains(&sender.name().to_string()));
        assert!(decoded.contains(&email.subject().to_string()));
        assert!(decoded.contains(&email.body().to_string()));
        for recipient in email
            .public_recipients()
            .into_iter()
            .chain(email.cc_recipients())
        {
            assert!(
                decoded.contains(&recipient.to_string()),
                "Expected recipient {} to be in the email",
                recipient
            );
        }

        for recipient in email.bcc_recipients() {
            // BCC recipients should not be in the formatted email
            assert!(!decoded.contains(&recipient.to_string()));
        }

        assert!(decoded.contains("application/pdf"));
        assert!(decoded.contains(&BASE64_STANDARD.encode(Pdf::sample().as_ref())))
    }

    #[test]
    fn test_message_from_email_with_sender_without_attachment() {
        let email = Email::builder()
            .public_recipients(IndexSet::from_iter(vec![EmailAddress::sample_bob()]))
            .cc_recipients(IndexSet::from_iter(vec![EmailAddress::sample_carol()]))
            .bcc_recipients(IndexSet::from_iter(vec![EmailAddress::sample_erin()]))
            .subject("Sample Email Subject".to_string())
            .body("This is a sample email body.".to_string())
            .build();
        let sender = EmailAccount::sample_alice();
        let email_with_sender = EmailWithSender::builder()
            .email(email.clone())
            .sender(sender.clone())
            .build();
        let message = Message::try_from(email_with_sender).unwrap();
        let encoded = message.formatted();
        let decoded = String::from_utf8(encoded).unwrap();
        assert!(decoded.contains(&sender.email().to_string()));
        assert!(decoded.contains(&sender.name().to_string()));
        assert!(decoded.contains(&email.subject().to_string()));
        assert!(decoded.contains(&email.body().to_string()));
        for recipient in email
            .public_recipients()
            .into_iter()
            .chain(email.cc_recipients())
        {
            assert!(
                decoded.contains(&recipient.to_string()),
                "Expected recipient {} to be in the email",
                recipient
            );
        }

        for recipient in email.bcc_recipients() {
            // BCC recipients should not be in the formatted email
            assert!(!decoded.contains(&recipient.to_string()));
        }

        assert!(!decoded.contains("application/pdf"));
    }
}
