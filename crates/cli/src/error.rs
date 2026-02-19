use thiserror::Error;

/// Top-level error type for the `klirr` CLI crate.
#[derive(Debug, Error)]
pub enum CliError {
    /// Wraps errors coming from `klirr-core-invoice`.
    #[error(transparent)]
    Core(#[from] klirr_core_invoice::Error),

    /// Wraps errors coming from `klirr-render-pdf`.
    #[error(transparent)]
    Render(#[from] klirr_render_pdf::Error),

    /// Wraps errors originating from email-related terminal prompts.
    #[error(transparent)]
    EmailFromTui(#[from] EmailFromTuiError),

    /// Wraps errors originating from invoice-data terminal prompts.
    #[error(transparent)]
    InvoiceDataFromTui(#[from] InvalidInvoiceData),

    /// The user supplied an output path that does not exist.
    #[error("Specified output path does not exist: {path}")]
    SpecifiedOutputPathDoesNotExist { path: String },
}

/// Errors that can occur when collecting email data from terminal prompts.
#[derive(Debug, Error)]
pub enum EmailFromTuiError {
    /// The entered password and confirmation password did not match.
    #[error("Passwords do not match")]
    PasswordDoesNotMatch,

    /// The entered email password is shorter than the configured minimum.
    #[error(
        "Email password is too short, expected at least {min_length} characters, but found {actual_length}"
    )]
    EmailPasswordTooShort {
        min_length: usize,
        actual_length: usize,
    },

    /// The email body template could not be parsed into an atom template.
    #[error("Failed to parse email atom template: {underlying}")]
    EmailAtomTemplateError { underlying: String },

    /// A provided email address was invalid for a specific role.
    #[error("Invalid email address for: {role}, because: {underlying}")]
    InvalidEmailAddress { role: String, underlying: String },

    /// A provided display name was invalid for a specific email role.
    #[error("Invalid name for email for: {role}, because: {underlying}")]
    InvalidNameForEmail { role: String, underlying: String },

    /// A provided password was invalid for a specific email purpose.
    #[error("Invalid password for email {purpose}, because: {underlying}")]
    InvalidPasswordForEmail { purpose: String, underlying: String },

    /// A recipients list was empty where at least one recipient is required.
    #[error("Recipient addresses cannot be empty")]
    RecipientAddressesCannotBeEmpty,

    /// The SMTP server value could not be parsed or validated.
    #[error("Failed to parse SMTP Server, because: {underlying}")]
    InvalidSmtpServer { underlying: String },
}

impl EmailFromTuiError {
    /// Creates an [`EmailFromTuiError::EmailAtomTemplateError`] from a displayable cause.
    pub fn email_atom_template_error(underlying: impl std::fmt::Display) -> Self {
        Self::EmailAtomTemplateError {
            underlying: underlying.to_string(),
        }
    }

    /// Creates an [`EmailFromTuiError::InvalidSmtpServer`] from a displayable cause.
    pub fn invalid_smtp_server(underlying: impl std::fmt::Display) -> Self {
        Self::InvalidSmtpServer {
            underlying: underlying.to_string(),
        }
    }

    /// Returns a mapper suitable for `map_err` that wraps invalid email-address errors.
    pub fn invalid_email_address_for_role<E: std::fmt::Display>(
        role: impl std::fmt::Display,
    ) -> impl FnOnce(E) -> Self {
        let role = role.to_string();
        move |e| Self::InvalidEmailAddress {
            role,
            underlying: e.to_string(),
        }
    }

    /// Returns a mapper suitable for `map_err` that wraps invalid email-name errors.
    pub fn invalid_name_for_email_for_role<E: std::fmt::Display>(
        role: impl std::fmt::Display,
    ) -> impl FnOnce(E) -> Self {
        let role = role.to_string();
        move |e| Self::InvalidNameForEmail {
            role,
            underlying: e.to_string(),
        }
    }

    /// Returns a mapper suitable for `map_err` that wraps invalid email-password errors.
    pub fn invalid_password_for_email_purpose<E: std::fmt::Display>(
        purpose: impl std::fmt::Display,
    ) -> impl FnOnce(E) -> Self {
        let purpose = purpose.to_string();
        move |e| Self::InvalidPasswordForEmail {
            purpose,
            underlying: e.to_string(),
        }
    }
}

/// Errors describing invalid invoice data entered through terminal prompts.
#[derive(Debug, Error)]
pub enum InvalidInvoiceData {
    /// Building `CompanyInformation` from prompt input failed.
    #[error("Failed to build CompanyInformation from Terminal UI input, because: {reason}")]
    CompanyInformation { reason: String },

    /// Building `InvoiceInfo` from prompt input failed.
    #[error("Failed to build InvoiceInfo from Terminal UI input, because: {reason}")]
    InvoiceInfo { reason: String },

    /// Building `PaymentInfo` from prompt input failed.
    #[error("Failed to build PaymentInfo from Terminal UI input, because: {reason}")]
    PaymentInfo { reason: String },

    /// Building `ServiceFees` from prompt input failed.
    #[error("Failed to build ServiceFees from Terminal UI input, because: {reason}")]
    ServiceFees { reason: String },

    /// A date entered in the terminal UI was not a valid calendar date.
    #[error("Invalid date, underlying: {underlying}")]
    Date { underlying: String },
}

impl InvalidInvoiceData {
    /// Creates an [`InvalidInvoiceData::Date`] from a displayable parser error.
    pub fn invalid_date(underlying: impl std::fmt::Display) -> Self {
        Self::Date {
            underlying: underlying.to_string(),
        }
    }

    /// Creates an [`InvalidInvoiceData::CompanyInformation`] from a debug value.
    pub fn invalid_company_information(reason: impl std::fmt::Debug) -> Self {
        Self::CompanyInformation {
            reason: format!("{reason:?}"),
        }
    }

    /// Creates an [`InvalidInvoiceData::InvoiceInfo`] from a debug value.
    pub fn invalid_invoice_info(reason: impl std::fmt::Debug) -> Self {
        Self::InvoiceInfo {
            reason: format!("{reason:?}"),
        }
    }

    /// Creates an [`InvalidInvoiceData::PaymentInfo`] from a debug value.
    pub fn invalid_payment_info(reason: impl std::fmt::Debug) -> Self {
        Self::PaymentInfo {
            reason: format!("{reason:?}"),
        }
    }

    /// Creates an [`InvalidInvoiceData::ServiceFees`] from a debug value.
    pub fn invalid_service_fees(reason: impl std::fmt::Debug) -> Self {
        Self::ServiceFees {
            reason: format!("{reason:?}"),
        }
    }
}

/// Backward-compatible alias used in existing callsites.
pub type InvoiceDataFromTuiError = InvalidInvoiceData;

/// Generic CLI `Result` alias.
pub type Result<T, E = CliError> = std::result::Result<T, E>;
/// Convenience alias for CLI operations that return [`CliError`].
pub type CliResult<T> = Result<T, CliError>;

#[cfg(test)]
mod tests {
    use super::{EmailFromTuiError, InvalidInvoiceData};
    use std::fmt;

    struct DebugPassthrough(&'static str);
    impl fmt::Debug for DebugPassthrough {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn email_atom_template_error_keeps_underlying_message() {
        let err = EmailFromTuiError::email_atom_template_error("bad template");
        assert!(matches!(
            err,
            EmailFromTuiError::EmailAtomTemplateError { underlying } if underlying == "bad template"
        ));
    }

    #[test]
    fn invalid_smtp_server_keeps_underlying_message() {
        let err = EmailFromTuiError::invalid_smtp_server("bad smtp");
        assert!(matches!(
            err,
            EmailFromTuiError::InvalidSmtpServer { underlying } if underlying == "bad smtp"
        ));
    }

    #[test]
    fn invalid_email_address_for_role_mapper_sets_role_and_message() {
        let err = EmailFromTuiError::invalid_email_address_for_role("Sender")("not-an-email");
        assert!(matches!(
            err,
            EmailFromTuiError::InvalidEmailAddress { role, underlying }
                if role == "Sender" && underlying == "not-an-email"
        ));
    }

    #[test]
    fn invalid_name_for_email_for_role_mapper_sets_role_and_message() {
        let err =
            EmailFromTuiError::invalid_name_for_email_for_role("Reply-To")("name parse failed");
        assert!(matches!(
            err,
            EmailFromTuiError::InvalidNameForEmail { role, underlying }
                if role == "Reply-To" && underlying == "name parse failed"
        ));
    }

    #[test]
    fn invalid_password_for_email_purpose_mapper_sets_purpose_and_message() {
        let err =
            EmailFromTuiError::invalid_password_for_email_purpose("SMTP app password")("too short");
        assert!(matches!(
            err,
            EmailFromTuiError::InvalidPasswordForEmail { purpose, underlying }
                if purpose == "SMTP app password" && underlying == "too short"
        ));
    }

    #[test]
    fn invalid_date_keeps_underlying_message() {
        let err = InvalidInvoiceData::invalid_date("invalid date");
        assert!(matches!(
            err,
            InvalidInvoiceData::Date { underlying } if underlying == "invalid date"
        ));
    }

    #[test]
    fn invalid_company_information_keeps_reason_message() {
        let err =
            InvalidInvoiceData::invalid_company_information(DebugPassthrough("company invalid"));
        assert!(matches!(
            err,
            InvalidInvoiceData::CompanyInformation { reason } if reason == "company invalid"
        ));
    }

    #[test]
    fn invalid_invoice_info_keeps_reason_message() {
        let err = InvalidInvoiceData::invalid_invoice_info(DebugPassthrough("invoice invalid"));
        assert!(matches!(
            err,
            InvalidInvoiceData::InvoiceInfo { reason } if reason == "invoice invalid"
        ));
    }

    #[test]
    fn invalid_payment_info_keeps_reason_message() {
        let err = InvalidInvoiceData::invalid_payment_info(DebugPassthrough("payment invalid"));
        assert!(matches!(
            err,
            InvalidInvoiceData::PaymentInfo { reason } if reason == "payment invalid"
        ));
    }

    #[test]
    fn invalid_service_fees_keeps_reason_message() {
        let err = InvalidInvoiceData::invalid_service_fees(DebugPassthrough("fees invalid"));
        assert!(matches!(
            err,
            InvalidInvoiceData::ServiceFees { reason } if reason == "fees invalid"
        ));
    }
}
