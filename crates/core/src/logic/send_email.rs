use crate::prelude::*;

use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

pub fn send_email_with_settings_for_pdf(
    named_pdf: &NamedPdf,
    email_settings: &DecryptedEmailSettings,
) -> Result<()> {
    let (email, credentials) = email_settings.compose(named_pdf);
    send_email_with_credentials(email, credentials)
}

/// Sends an email using the provided credentials using lettre crate.
fn send_email_with_credentials(email: Email, credentials: EmailCredentials) -> Result<()> {
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
