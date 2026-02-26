use crate::{Currency, Granularity, Language, Version};
use thiserror::Error as ThisError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error type for the logic crate, encapsulating various errors that can occur
/// during PDF generation and manipulation.
#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum Error {
    /// The offset period must not be in the record of periods off.
    #[error("Records off must not contain offset period: {offset_period}")]
    RecordsOffMustNotContainOffsetPeriod {
        /// The offset period represented as a date string.
        offset_period: String,
    },

    /// The start period is after the end period.
    #[error("Start period ('{start}') is after end period ('{end}')")]
    StartPeriodAfterEndPeriod {
        /// Start period label.
        start: String,
        /// End period label.
        end: String,
    },

    /// Invalid period input string/value.
    #[error("Invalid Period, bad value: {bad_value}")]
    InvalidPeriod {
        /// User-provided period text that failed validation/parsing.
        bad_value: String,
    },

    /// Time-off granularity does not match service fee granularity.
    #[error(
        "Invalid granularity for time off: '{free_granularity}', expected: '{service_fees_granularity}', use the same time unit for time off as you specified in service fees. View it with `klirr data dump` command."
    )]
    InvalidGranularityForTimeOff {
        /// The granularity supplied for time off.
        free_granularity: Granularity,
        /// The expected granularity derived from service fee configuration.
        service_fees_granularity: Granularity,
    },

    /// Granularity too coarse,
    #[error(
        "Granularity too coarse '{granularity}', max is: '{max_granularity}', for period: '{target_period}'"
    )]
    GranularityTooCoarse {
        /// Requested granularity that is too coarse for the cadence.
        granularity: Granularity,
        /// Maximum allowed granularity for the active cadence.
        max_granularity: Granularity,
        /// Target period label for context.
        target_period: String,
    },

    /// Cannot invoice for month when cadence is bi-weekly.
    #[error("Cannot invoice for month when cadence is bi-weekly")]
    CannotInvoiceForMonthWhenCadenceIsBiWeekly,

    /// Cannot invoice for fortnight when cadence is monthly.
    #[error("Cannot invoice for fortnight when cadence is monthly")]
    CannotInvoiceForFortnightWhenCadenceIsMonthly,

    /// Cannot expense for month when cadence is bi-weekly
    #[error("Cannot expense for month when cadence is bi-weekly")]
    CannotExpenseForMonthWhenCadenceIsBiWeekly,

    /// Cannot expense for fortnight when cadence is monthly.
    #[error("Cannot expense for fortnight when cadence is monthly")]
    CannotExpenseForFortnightWhenCadenceIsMonthly,

    /// Failed to parse a string into a valid UTF-8 string.
    #[error("Failed to parse string into a valid UTF-8 string")]
    InvalidUtf8,

    /// Failed to decrypt data with AES.
    #[error("Failed to decrypt data with AES")]
    AESDecryptionFailed,

    /// Invalid AES bytes, e.g. when the length is not as expected.
    #[error(
        "Invalid AES bytes, expected at least {expected_at_least} bytes, but found {found} bytes"
    )]
    InvalidAESBytesTooShort {
        /// Minimum expected byte length.
        expected_at_least: usize,
        /// Actual byte length found.
        found: usize,
    },

    /// Failed to create SMTP transport, e.g. when the SMTP server is not reachable.
    #[error("Failed to create SMTP transport, because: {underlying}")]
    CreateSmtpTransportError {
        /// Underlying SMTP transport error message.
        underlying: String,
    },

    /// Failed to create Lettre Email from Email struct.
    #[error("Failed to create email, because: {underlying}")]
    CreateEmailError {
        /// Underlying email construction error message.
        underlying: String,
    },

    /// Failed to add attachments to the email, e.g. when the file is not found or cannot be read.
    #[error("Failed to add attachments to the email, because: {underlying}")]
    AddAttachmentsError {
        /// Underlying error when adding attachments to the email.
        underlying: String,
    },

    /// Failed to send email
    #[error("Failed to send email, because: {underlying}")]
    SendEmailError {
        /// Underlying email send error message.
        underlying: String,
    },

    /// Failed to convert to `f64` from a `Decimal`
    #[error("Failed to convert to f64 from Decimal, because: {value}")]
    InvalidDecimalToF64Conversion {
        /// Decimal value that failed conversion.
        value: String,
    },

    /// Failed to convert `f64` value to a `Decimal`
    #[error("Failed to convert f64 to Decimal, because: {value}")]
    InvalidDecimalFromF64Conversion {
        /// Source float value that failed conversion.
        value: f64,
    },

    /// Failed to parse a string into an `Decimal`, e.g. when the string is not a valid number.
    #[error("Failed to parse f64 from string: {bad_value}, reason: {reason}")]
    InvalidF64String {
        /// String that failed to parse as f64.
        bad_value: String,
        /// Parsing failure reason.
        reason: String,
    },

    /// Failed to write data to disk, e.g. when the file system is not accessible.
    #[error("Failed to write data to disk, because: {underlying}")]
    FailedToWriteDataToDisk {
        /// Underlying IO/serialization error message.
        underlying: String,
    },

    /// Failed to serialize data to RON format.
    #[error("Failed to RON serialize data, because: {underlying}")]
    FailedToRonSerializeData {
        /// Type name that failed serialization.
        type_name: String,
        /// Underlying serialization error message.
        underlying: String,
    },

    /// Failed to parse invoice number from a string, e.g. when the format is incorrect.
    #[error("Failed to parse invoice number from string: {invalid_string}")]
    InvalidInvoiceNumberString {
        /// String that failed invoice-number parsing.
        invalid_string: String,
    },

    /// The offset period must not be in the record of periods off.
    #[error(
        "Offset period must not be in the record of periods off: {offset_period}, period kind: {period_kind}"
    )]
    OffsetPeriodMustNotBeInRecordOfPeriodsOff {
        /// Offset period label.
        offset_period: String,
        /// Period kind/type name for error context.
        period_kind: String,
    },

    /// Failed to create the output directory for the PDF file.
    #[error("Failed to create output directory: {underlying}")]
    FailedToCreateOutputDirectory {
        /// Underlying filesystem error message.
        underlying: String,
    },

    /// Target period must have expenses, but it does not.
    #[error(
        "Target period {target_period} must have expenses, but it does not. Fill 
    in the `input/data/expenses.json` file with expenses for this period."
    )]
    TargetPeriodMustHaveExpenses {
        /// Target period label with missing expenses.
        target_period: String,
    },

    /// Failed to parse year
    #[error("Failed to parse year: {invalid_string}")]
    FailedToParseYear {
        /// String that failed year parsing.
        invalid_string: String,
    },

    /// Failed to load file
    #[error("Failed to load file: {path}, underlying: {underlying}")]
    FileNotFound {
        /// File path that could not be loaded.
        path: String,
        /// Underlying IO error message.
        underlying: String,
    },

    /// Data schema version does not match the currently supported version.
    #[error(
        "Data version mismatch, found: {found}, expected: {current}. Your data must be manually migrated."
    )]
    DataVersionMismatch {
        /// Version found on disk.
        found: Version,
        /// Currently supported version.
        current: Version,
    },

    /// Failed to deserialize a type
    #[error("Failed to deserialize {type_name}, because: {error}")]
    Deserialize {
        /// Type name that failed deserialization.
        type_name: String,
        /// Underlying deserialization error message.
        error: String,
    },

    /// Failed to parse Day from String
    #[error("Invalid day from String: {invalid_string}, reason: {reason}")]
    InvalidDayFromString {
        /// String that failed day parsing.
        invalid_string: String,
        /// Parsing failure reason.
        reason: String,
    },

    /// Invalid date
    #[error("Invalid date, underlying: {underlying}")]
    InvalidDate {
        /// Underlying invalid-date reason.
        underlying: String,
    },

    /// Invalid day of the month, e.g. when the day is not between 1 and 31.
    #[error("Invalid day: {day}, reason: {reason}")]
    InvalidDay {
        /// Invalid day value.
        day: i32,
        /// Validation failure reason.
        reason: String,
    },

    /// Invalid month, e.g. when the month is not between 1 and 12.
    #[error("Invalid month: {month}, reason: {reason}")]
    InvalidMonth {
        /// Invalid month value.
        month: i32,
        /// Validation failure reason.
        reason: String,
    },

    /// Failed to parse Month from String
    #[error("Failed to parse Month: {invalid_string}")]
    FailedToParseMonth {
        /// String that failed month parsing.
        invalid_string: String,
    },

    /// Failed to parse expense item from a string, e.g. when the format is incorrect.
    #[error("Failed to parse expense item from: '{invalid_string}': {reason}")]
    InvalidExpenseItem {
        /// Raw item text that failed parsing.
        invalid_string: String,
        /// Parsing failure reason.
        reason: String,
    },

    /// The target period is in the record of periods off, but it must not be.
    #[error("Target period {target_period} is in the record of periods off, but it must not be.")]
    TargetPeriodMustNotBeInRecordOfPeriodsOff {
        /// Target period that was unexpectedly marked as off.
        target_period: String,
    },

    /// Failed to parse PaymentTerms NetDays from a string, e.g. when the format is incorrect.
    #[error("Failed to PaymentTerms NetDays from string: {invalid_string}")]
    FailedToParsePaymentTermsNetDays {
        /// String that failed net-days parsing.
        invalid_string: String,
    },

    /// Failed to find the localization file for a specific language.
    #[error("Failed to find the localization file for language: {language}")]
    L10nNotFound {
        /// The language that was not found, e.g. "EN" for English.
        language: Language,
    },

    /// Failed to parse a string into a Hexcolor
    #[error("Invalid hex color format: {invalid_string}")]
    InvalidHexColor {
        /// String that failed hex-color parsing.
        invalid_string: String,
    },

    /// Failed to parse a date, e.g. when the format is incorrect or the date is invalid.
    #[error("Failed to parse date, because: {underlying}")]
    FailedToParseDate {
        /// Underlying date-parse error message.
        underlying: String,
    },

    /// Error converting between currencies, e.g. when the exchange rate is not found.
    #[error("Found no exchange rate for {target} based on {base}")]
    FoundNoExchangeRate {
        /// The target currency for the exchange rate, e.g. "EUR".
        target: Currency,
        /// The base currency for the exchange rate, e.g. "USD".
        base: Currency,
    },

    /// Error when saving the PDF to a file.
    #[error("Failed to save PDF, because: {underlying}")]
    SavePdf {
        /// Underlying PDF save failure reason.
        underlying: String,
    },

    /// Error when fetching exchange rates from an API.
    #[error("Failed fetch exchange rate from API, because: {underlying}")]
    NetworkError {
        /// Underlying network error message.
        underlying: String,
    },

    /// Error when parsing the response from the exchange rate API.
    #[error("Failed to parse exchange rate response, because: {underlying}")]
    ParseError {
        /// Underlying response parse error message.
        underlying: String,
    },
}

impl Error {
    /// Creates a [`Error::CreateSmtpTransportError`] from a debug-formatted source error.
    pub fn create_smtp_transport_error(underlying: impl std::fmt::Debug) -> Self {
        Self::CreateSmtpTransportError {
            underlying: format!("Failed to create SMTP transport: {underlying:?}"),
        }
    }

    /// Creates a [`Error::SendEmailError`] from a debug-formatted source error.
    pub fn send_email_error(underlying: impl std::fmt::Debug) -> Self {
        Self::SendEmailError {
            underlying: format!("Failed to send email: {underlying:?}"),
        }
    }

    /// Creates a [`Error::CreateEmailError`] from a debug-formatted source error.
    pub fn create_email_error(underlying: impl std::fmt::Debug) -> Self {
        Self::CreateEmailError {
            underlying: format!("{underlying:?}"),
        }
    }

    /// Creates a [`Error::FailedToCreateOutputDirectory`] from a debug-formatted source error.
    pub fn failed_to_create_output_directory(underlying: impl std::fmt::Debug) -> Self {
        Self::FailedToCreateOutputDirectory {
            underlying: format!("{underlying:?}"),
        }
    }

    /// Creates a [`Error::SavePdf`] from a string-like source error.
    pub fn save_pdf(underlying: impl Into<String>) -> Self {
        Self::SavePdf {
            underlying: underlying.into(),
        }
    }

    /// Creates a [`Error::FailedToWriteDataToDisk`] from a debug-formatted source error.
    pub fn failed_to_write_data_to_disk(underlying: impl std::fmt::Debug) -> Self {
        Self::FailedToWriteDataToDisk {
            underlying: format!("{underlying:?}"),
        }
    }

    /// Returns a `map_err` helper that constructs [`Error::FailedToRonSerializeData`].
    pub fn failed_to_ron_serialize_data<E: std::fmt::Debug>(
        type_name: impl Into<String>,
    ) -> impl FnOnce(E) -> Self {
        let type_name = type_name.into();
        move |error| Self::FailedToRonSerializeData {
            type_name,
            underlying: format!("{error:?}"),
        }
    }

    /// Returns a `map_err` helper that constructs [`Error::FileNotFound`].
    pub fn file_not_found<E: std::fmt::Debug>(path: impl Into<String>) -> impl FnOnce(E) -> Self {
        let path = path.into();
        move |error| Self::FileNotFound {
            path,
            underlying: format!("{error:?}"),
        }
    }

    /// Creates a [`Error::DataVersionMismatch`] from found/current versions.
    pub fn data_version_mismatch(found: Version, current: Version) -> Self {
        Self::DataVersionMismatch { found, current }
    }

    /// Returns a `map_err` helper that constructs [`Error::Deserialize`].
    pub fn deserialize<E: std::fmt::Display>(
        type_name: impl Into<String>,
    ) -> impl FnOnce(E) -> Self {
        let type_name = type_name.into();
        move |error| Self::Deserialize {
            type_name,
            error: error.to_string(),
        }
    }

    /// Returns a `map_err` helper that constructs [`Error::ParseError`].
    pub fn parse_error<E: std::fmt::Display>(context: impl Into<String>) -> impl FnOnce(E) -> Self {
        let context = context.into();
        move |error| Self::ParseError {
            underlying: format!("{context}: {error}"),
        }
    }

    /// Returns a `map_err` helper that constructs [`Error::NetworkError`].
    pub fn network_error<E: std::fmt::Display>(
        context: impl Into<String>,
    ) -> impl FnOnce(E) -> Self {
        let context = context.into();
        move |error| Self::NetworkError {
            underlying: format!("{context}: {error}"),
        }
    }

    /// Returns a `map_err` helper that constructs [`Error::InvalidExpenseItem`]
    /// for a specific input string and field.
    pub fn invalid_expense_item<E: std::fmt::Display>(
        invalid_string: impl Into<String>,
        field: impl Into<String>,
    ) -> impl FnOnce(E) -> Self {
        let invalid_string = invalid_string.into();
        let field = field.into();
        move |error| Self::InvalidExpenseItem {
            invalid_string,
            reason: format!("Failed to parse {field}: {error}"),
        }
    }

    /// Maps a decryption failure to [`Error::AESDecryptionFailed`].
    pub fn aes_decryption_failed(_error: impl std::fmt::Debug) -> Self {
        Self::AESDecryptionFailed
    }
}

impl From<klirr_foundation::CryptoError> for Error {
    fn from(value: klirr_foundation::CryptoError) -> Self {
        match value {
            klirr_foundation::CryptoError::InvalidUtf8 => Self::InvalidUtf8,
            klirr_foundation::CryptoError::AesDecryptionFailed => Self::AESDecryptionFailed,
            klirr_foundation::CryptoError::InvalidAesBytesTooShort {
                expected_at_least,
                found,
            } => Self::InvalidAESBytesTooShort {
                expected_at_least,
                found,
            },
        }
    }
}

impl From<klirr_foundation::ModelError> for Error {
    fn from(value: klirr_foundation::ModelError) -> Self {
        match value {
            klirr_foundation::ModelError::InvalidDecimalToF64Conversion { value } => {
                Self::InvalidDecimalToF64Conversion { value }
            }
            klirr_foundation::ModelError::InvalidDecimalFromF64Conversion { value } => {
                Self::InvalidDecimalFromF64Conversion { value }
            }
            klirr_foundation::ModelError::FailedToParseYear { invalid_string } => {
                Self::FailedToParseYear { invalid_string }
            }
            klirr_foundation::ModelError::InvalidDayFromString {
                invalid_string,
                reason,
            } => Self::InvalidDayFromString {
                invalid_string,
                reason,
            },
            klirr_foundation::ModelError::InvalidDay { day, reason } => {
                Self::InvalidDay { day, reason }
            }
            klirr_foundation::ModelError::InvalidMonth { month, reason } => {
                Self::InvalidMonth { month, reason }
            }
            klirr_foundation::ModelError::FailedToParseMonth { invalid_string } => {
                Self::FailedToParseMonth { invalid_string }
            }
            klirr_foundation::ModelError::FailedToParseDate { underlying } => {
                Self::FailedToParseDate { underlying }
            }
            klirr_foundation::ModelError::InvalidHexColor { invalid_string } => {
                Self::InvalidHexColor { invalid_string }
            }
            klirr_foundation::ModelError::InvalidDate { underlying } => {
                Self::InvalidDate { underlying }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use crate::Version;
    use std::fmt;

    struct DebugPassthrough(&'static str);
    impl fmt::Debug for DebugPassthrough {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    struct DisplayPassthrough(&'static str);
    impl fmt::Display for DisplayPassthrough {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn create_smtp_transport_error_keeps_message() {
        let err = Error::create_smtp_transport_error(DebugPassthrough("smtp-fail"));
        assert!(matches!(
            err,
            Error::CreateSmtpTransportError { underlying }
                if underlying == "Failed to create SMTP transport: smtp-fail"
        ));
    }

    #[test]
    fn send_email_error_keeps_message() {
        let err = Error::send_email_error(DebugPassthrough("send-fail"));
        assert!(matches!(
            err,
            Error::SendEmailError { underlying } if underlying == "Failed to send email: send-fail"
        ));
    }

    #[test]
    fn create_email_error_keeps_message() {
        let err = Error::create_email_error(DebugPassthrough("mailbox parse failed"));
        assert!(matches!(
            err,
            Error::CreateEmailError { underlying } if underlying == "mailbox parse failed"
        ));
    }

    #[test]
    fn failed_to_create_output_directory_keeps_message() {
        let err = Error::failed_to_create_output_directory(DebugPassthrough("permission denied"));
        assert!(matches!(
            err,
            Error::FailedToCreateOutputDirectory { underlying } if underlying == "permission denied"
        ));
    }

    #[test]
    fn save_pdf_keeps_message() {
        let err = Error::save_pdf("disk full");
        assert!(matches!(
            err,
            Error::SavePdf { underlying } if underlying == "disk full"
        ));
    }

    #[test]
    fn failed_to_write_data_to_disk_keeps_message() {
        let err = Error::failed_to_write_data_to_disk(DebugPassthrough("read-only file system"));
        assert!(matches!(
            err,
            Error::FailedToWriteDataToDisk { underlying } if underlying == "read-only file system"
        ));
    }

    #[test]
    fn failed_to_ron_serialize_data_mapper_sets_type_and_message() {
        let err = Error::failed_to_ron_serialize_data("MyType")(DebugPassthrough("serialize fail"));
        assert!(matches!(
            err,
            Error::FailedToRonSerializeData { type_name, underlying }
                if type_name == "MyType" && underlying == "serialize fail"
        ));
    }

    #[test]
    fn file_not_found_mapper_sets_path_and_message() {
        let err = Error::file_not_found("/tmp/missing.ron")(DebugPassthrough("no such file"));
        assert!(matches!(
            err,
            Error::FileNotFound { path, underlying }
                if path == "/tmp/missing.ron" && underlying == "no such file"
        ));
    }

    #[test]
    fn data_version_mismatch_sets_versions() {
        let err = Error::data_version_mismatch(Version::V0, Version::current());
        assert!(matches!(
            err,
            Error::DataVersionMismatch { found, current }
                if found == Version::V0 && current == Version::current()
        ));
    }

    #[test]
    fn deserialize_mapper_sets_type_and_message() {
        let err = Error::deserialize("MyType")(DisplayPassthrough("expected struct"));
        assert!(matches!(
            err,
            Error::Deserialize { type_name, error } if type_name == "MyType" && error == "expected struct"
        ));
    }

    #[test]
    fn parse_error_mapper_sets_context_and_message() {
        let err = Error::parse_error("Parse JSON")(DisplayPassthrough("eof"));
        assert!(matches!(
            err,
            Error::ParseError { underlying } if underlying == "Parse JSON: eof"
        ));
    }

    #[test]
    fn network_error_mapper_sets_context_and_message() {
        let err = Error::network_error("Fetch rate")(DisplayPassthrough("timed out"));
        assert!(matches!(
            err,
            Error::NetworkError { underlying } if underlying == "Fetch rate: timed out"
        ));
    }

    #[test]
    fn invalid_expense_item_mapper_sets_input_field_and_message() {
        let err = Error::invalid_expense_item("Coffee,abc,EUR,1,2025-01-01", "unit_price")(
            DisplayPassthrough("invalid decimal"),
        );
        assert!(matches!(
            err,
            Error::InvalidExpenseItem { invalid_string, reason }
                if invalid_string == "Coffee,abc,EUR,1,2025-01-01"
                    && reason == "Failed to parse unit_price: invalid decimal"
        ));
    }

    #[test]
    fn aes_decryption_failed_maps_to_expected_variant() {
        let err = Error::aes_decryption_failed(DebugPassthrough("tag mismatch"));
        assert!(matches!(err, Error::AESDecryptionFailed));
    }

    #[test]
    fn from_crypto_error_maps_all_variants() {
        let invalid_utf8: Error = klirr_foundation::CryptoError::InvalidUtf8.into();
        assert_eq!(invalid_utf8, Error::InvalidUtf8);

        let aes_decryption_failed: Error =
            klirr_foundation::CryptoError::AesDecryptionFailed.into();
        assert_eq!(aes_decryption_failed, Error::AESDecryptionFailed);

        let invalid_too_short: Error = klirr_foundation::CryptoError::InvalidAesBytesTooShort {
            expected_at_least: 32,
            found: 7,
        }
        .into();
        assert_eq!(
            invalid_too_short,
            Error::InvalidAESBytesTooShort {
                expected_at_least: 32,
                found: 7,
            }
        );
    }

    #[test]
    fn from_model_error_maps_all_variants() {
        let err: Error = klirr_foundation::ModelError::InvalidDecimalToF64Conversion {
            value: "12.34".to_string(),
        }
        .into();
        assert_eq!(
            err,
            Error::InvalidDecimalToF64Conversion {
                value: "12.34".to_string()
            }
        );

        let err: Error =
            klirr_foundation::ModelError::InvalidDecimalFromF64Conversion { value: 99.5 }.into();
        assert_eq!(err, Error::InvalidDecimalFromF64Conversion { value: 99.5 });

        let err: Error = klirr_foundation::ModelError::FailedToParseYear {
            invalid_string: "20xx".to_string(),
        }
        .into();
        assert_eq!(
            err,
            Error::FailedToParseYear {
                invalid_string: "20xx".to_string()
            }
        );

        let err: Error = klirr_foundation::ModelError::InvalidDayFromString {
            invalid_string: "foo".to_string(),
            reason: "not a number".to_string(),
        }
        .into();
        assert_eq!(
            err,
            Error::InvalidDayFromString {
                invalid_string: "foo".to_string(),
                reason: "not a number".to_string(),
            }
        );

        let err: Error = klirr_foundation::ModelError::InvalidDay {
            day: 99,
            reason: "out of range".to_string(),
        }
        .into();
        assert_eq!(
            err,
            Error::InvalidDay {
                day: 99,
                reason: "out of range".to_string(),
            }
        );

        let err: Error = klirr_foundation::ModelError::InvalidMonth {
            month: 13,
            reason: "out of range".to_string(),
        }
        .into();
        assert_eq!(
            err,
            Error::InvalidMonth {
                month: 13,
                reason: "out of range".to_string(),
            }
        );

        let err: Error = klirr_foundation::ModelError::FailedToParseMonth {
            invalid_string: "Smarch".to_string(),
        }
        .into();
        assert_eq!(
            err,
            Error::FailedToParseMonth {
                invalid_string: "Smarch".to_string()
            }
        );

        let err: Error = klirr_foundation::ModelError::FailedToParseDate {
            underlying: "bad format".to_string(),
        }
        .into();
        assert_eq!(
            err,
            Error::FailedToParseDate {
                underlying: "bad format".to_string()
            }
        );

        let err: Error = klirr_foundation::ModelError::InvalidDate {
            underlying: "day out of range".to_string(),
        }
        .into();
        assert_eq!(
            err,
            Error::InvalidDate {
                underlying: "day out of range".to_string()
            }
        );
    }
}
