use inquire::CustomType;

use crate::{Error, Result, SmtpServer};

pub fn ask_for_smtp_server(default: &SmtpServer) -> Result<SmtpServer> {
    CustomType::<SmtpServer>::new("SMTP server?")
        .with_help_message("The SMTP server to use for sending emails")
        .with_default(default.clone())
        .prompt()
        .map_err(|e| Error::InvalidSmtpServer {
            underlying: e.to_string(),
        })
}
