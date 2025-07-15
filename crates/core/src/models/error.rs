use crate::prelude::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error type for the logic crate, encapsulating various errors that can occur
/// during PDF generation and manipulation.
#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum Error {
    /// Not a valid YearAndMonth nor YearMonthAndFortnight
    #[error("Invalid Period, bad value: {bad_value}")]
    InvalidPeriod { bad_value: String },

    /// Period is not YearAndMonth
    #[error("Period is not YearAndMonth")]
    PeriodIsNotYearAndMonth,

    /// Period is not YearMonthAndFortnight
    #[error("Period is not YearMonthAndFortnight")]
    PeriodIsNotYearMonthAndFortnight,

    #[error(
        "Invalid granularity for time off: '{free_granularity}', expected: '{service_fees_granularity}', use the same time unit for time off as you specified in service fees. View it with `klirr data dump` command."
    )]
    InvalidGranularityForTimeOff {
        free_granularity: Granularity,
        service_fees_granularity: Granularity,
    },

    /// Granularity too coarse,
    #[error(
        "Granularity too coarse '{granularity}', max is: '{max_granularity}', for period: '{target_period}'"
    )]
    GranularityTooCoarse {
        granularity: Granularity,
        max_granularity: Granularity,
        target_period: String,
    },

    /// Cannot invoice for month when cadence is bi-weekly.
    #[error("Cannot invoice for month when cadence is bi-weekly")]
    CannotInvoiceForMonthWhenCadenceIsBiWeekly,

    /// Cannot expense for month when cadence is bi-weekly
    #[error("Cannot expense for month when cadence is bi-weekly")]
    CannotExpenseForMonthWhenCadenceIsBiWeekly,

    /// Cannot expense for fortnight when cadence is monthly.
    #[error("Cannot expense for fortnight when cadence is monthly")]
    CannotExpenseForFortnightWhenCadenceIsMonthly,

    /// Password does not match, e.g. when the user tries to set a password
    /// and the confirmation password does not match.
    #[error("Passwords do not match")]
    PasswordDoesNotMatch,

    /// Email password is too short.
    #[error(
        "Email password is too short, expected at least {min_length} characters, but found {actual_length}"
    )]
    EmailPasswordTooShort {
        /// The minimum length of the email password.
        min_length: usize,
        /// The actual length of the email password.
        actual_length: usize,
    },

    /// Failed to parse the email atom template, e.g. when the template is not valid.
    #[error("Failed to parse email atom template: {underlying}")]
    EmailAtomTemplateError { underlying: String },

    /// Invalid email address
    #[error("Invalid email address for: {role}, because: {underlying}")]
    InvalidEmailAddress { role: String, underlying: String },

    /// Invalid name for email
    #[error("Invalid name for email for: {role}, because: {underlying}")]
    InvalidNameForEmail { role: String, underlying: String },

    #[error("Invalid password for email {purpose}, because: {underlying}")]
    InvalidPasswordForEmail { purpose: String, underlying: String },

    /// Recipient addresses cannot be empty.
    #[error("Recipient addresses cannot be empty")]
    RecipientAddressesCannotBeEmpty,

    /// Failed to parse SMTP Server
    #[error("Failed to parse SMTP Server, because: {underlying}")]
    InvalidSmtpServer { underlying: String },

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
        expected_at_least: usize,
        found: usize,
    },

    /// Failed to create SMTP transport, e.g. when the SMTP server is not reachable.
    #[error("Failed to create SMTP transport, because: {underlying}")]
    CreateSmtpTransportError { underlying: String },

    /// Failed to create Lettre Email from Email struct.
    #[error("Failed to create email, because: {underlying}")]
    CreateEmailError { underlying: String },

    /// Failed to add attachments to the email, e.g. when the file is not found or cannot be read.
    #[error("Failed to add attachments to the email, because: {underlying}")]
    AddAttachmentsError {
        /// Underlying error when adding attachments to the email.
        underlying: String,
    },

    /// Failed to send email
    #[error("Failed to send email, because: {underlying}")]
    SendEmailError { underlying: String },

    /// Failed to convert to `f64` from a `Decimal`
    #[error("Failed to convert to f64 from Decimal, because: {value}")]
    InvalidDecimalToF64Conversion { value: String },

    /// Failed to convert `f64` value to a `Decimal`
    #[error("Failed to convert f64 to Decimal, because: {value}")]
    InvalidDecimalFromF64Conversion { value: f64 },

    /// Failed to load a font, e.g. when the font file is not found or cannot be read.
    #[error("Failed to load font with family name: '{family_name}'")]
    FailedToLoadFont { family_name: String },

    /// Failed to parse a string into an `Decimal`, e.g. when the string is not a valid number.
    #[error("Failed to parse f64 from string: {bad_value}, reason: {reason}")]
    InvalidF64String { bad_value: String, reason: String },

    /// Failed to write data to disk, e.g. when the file system is not accessible.
    #[error("Failed to write data to disk, because: {underlying}")]
    FailedToWriteDataToDisk { underlying: String },

    /// Failed to serialize data to RON format.
    #[error("Failed to RON serialize data, because: {underlying}")]
    FailedToRonSerializeData {
        type_name: String,
        underlying: String,
    },

    /// Error while building CompanyInformation from Terminal UI input.
    #[error("Failed to build CompanyInformation from Terminal UI input, because: {reason}")]
    InvalidCompanyInformation { reason: String },

    /// Failed to parse invoice number from a string, e.g. when the format is incorrect.
    #[error("Failed to parse invoice number from string: {invalid_string}")]
    InvalidInvoiceNumberString { invalid_string: String },

    /// Error while building InvoiceInfo from Terminal UI input.
    #[error("Failed to build InvoiceInfo from Terminal UI input, because: {reason}")]
    InvalidInvoiceInfo { reason: String },

    /// Error while building PaymentInfo from Terminal UI input.
    #[error("Failed to build PaymentInfo from Terminal UI input, because: {reason}")]
    InvalidPaymentInfo { reason: String },

    /// Error while building ServiceFees from Terminal UI input.
    #[error("Failed to build ServiceFees from Terminal UI input, because: {reason}")]
    InvalidServiceFees { reason: String },

    /// The offset period must not be in the record of periods off.
    #[error(
        "Offset period must not be in the record of periods off: {offset_period}, period kind: {period_kind}"
    )]
    OffsetPeriodMustNotBeInRecordOfPeriodsOff {
        offset_period: String,
        period_kind: String,
    },

    /// The manually specified output path does not exist.
    #[error("Specified output path does not exist: {path}")]
    SpecifiedOutputPathDoesNotExist { path: String },

    /// Failed to create the output directory for the PDF file.
    #[error("Failed to create output directory: {underlying}")]
    FailedToCreateOutputDirectory { underlying: String },

    /// Target period must have expenses, but it does not.
    #[error(
        "Target period {target_period} must have expenses, but it does not. Fill 
    in the `input/data/expenses.json` file with expenses for this period."
    )]
    TargetPeriodMustHaveExpenses { target_period: String },

    /// Failed to parse year
    #[error("Failed to parse year: {invalid_string}")]
    FailedToParseYear { invalid_string: String },

    /// Failed to load file
    #[error("Failed to load file: {path}, underlying: {underlying}")]
    FileNotFound { path: String, underlying: String },

    /// Failed to deserialize a type
    #[error("Failed to deserialize {type_name}, because: {error}")]
    Deserialize { type_name: String, error: String },

    /// Failed to parse Day from String
    #[error("Invalid day from String: {invalid_string}, reason: {reason}")]
    InvalidDayFromString {
        invalid_string: String,
        reason: String,
    },

    /// Invalid YearAndMonth
    #[error("Invalid YearAndMonth, underlying: {underlying}")]
    InvalidYearAndMonth { underlying: String },

    /// Invalid date
    #[error("Invalid date, underlying: {underlying}")]
    InvalidDate { underlying: String },

    /// Invalid day of the month, e.g. when the day is not between 1 and 31.
    #[error("Invalid day: {day}, reason: {reason}")]
    InvalidDay { day: i32, reason: String },

    /// Invalid month, e.g. when the month is not between 1 and 12.
    #[error("Invalid month: {month}, reason: {reason}")]
    InvalidMonth { month: i32, reason: String },

    /// Failed to parse Month from String
    #[error("Failed to parse Month: {invalid_string}")]
    FailedToParseMonth { invalid_string: String },

    /// Failed to parse expense item from a string, e.g. when the format is incorrect.
    #[error("Failed to parse expense item from: '{invalid_string}': {reason}")]
    InvalidExpenseItem {
        invalid_string: String,
        reason: String,
    },

    /// The target period is in the record of periods off, but it must not be.
    #[error("Target period {target_period} is in the record of periods off, but it must not be.")]
    TargetPeriodMustNotBeInRecordOfPeriodsOff { target_period: String },

    /// Failed to parse PaymentTerms NetDays from a string, e.g. when the format is incorrect.
    #[error("Failed to PaymentTerms NetDays from string: {invalid_string}")]
    FailedToParsePaymentTermsNetDays { invalid_string: String },

    /// Failed to find the localization file for a specific language.
    #[error("Failed to find the localization file for language: {language}")]
    L18nNotFound {
        /// The language that was not found, e.g. "EN" for English.
        language: Language,
    },

    /// Failed to parse a string into a Hexcolor
    #[error("Invalid hex color format: {invalid_string}")]
    InvalidHexColor { invalid_string: String },

    /// Failed to parse a date, e.g. when the format is incorrect or the date is invalid.
    #[error("Failed to parse date, because: {underlying}")]
    FailedToParseDate { underlying: String },

    /// Error converting between currencies, e.g. when the exchange rate is not found.
    #[error("Found no exchange rate for {target} based on {base}")]
    FoundNoExchangeRate {
        /// The target currency for the exchange rate, e.g. "EUR".
        target: Currency,
        /// The base currency for the exchange rate, e.g. "USD".
        base: Currency,
    },

    /// Error when loading a resource for typst.
    #[error("Failed to load Typst source, because: {underlying}")]
    LoadSource { underlying: String },

    /// Error when compiling Typst source to a PagedDocument.
    #[error("Failed to compile Typst source, because: {underlying}")]
    BuildPdf { underlying: String },

    /// Error when exporting a PagedDocument to PDF.
    #[error("Failed to export PagedDocument to PDF, because: {underlying}")]
    ExportDocumentToPdf { underlying: String },

    /// Error when saving the PDF to a file.
    #[error("Failed to save PDF, because: {underlying}")]
    SavePdf { underlying: String },

    /// Error when fetching exchange rates from an API.
    #[error("Failed fetch exchange rate from API, because: {underlying}")]
    NetworkError { underlying: String },

    /// Error when parsing the response from the exchange rate API.
    #[error("Failed to parse exchange rate response, because: {underlying}")]
    ParseError { underlying: String },
}
