use thiserror::Error as ThisError;

pub type ModelResult<T, E = ModelError> = std::result::Result<T, E>;

/// Errors emitted by shared scalar/date models in foundation.
#[derive(Clone, Debug, PartialEq, ThisError)]
pub enum ModelError {
    /// Failed to convert to `f64` from `Decimal`.
    #[error("Failed to convert to f64 from Decimal, because: {value}")]
    InvalidDecimalToF64Conversion { value: String },

    /// Failed to convert `f64` into `Decimal`.
    #[error("Failed to convert f64 to Decimal, because: {value}")]
    InvalidDecimalFromF64Conversion { value: f64 },

    /// Failed to parse year.
    #[error("Failed to parse year: {invalid_string}")]
    FailedToParseYear { invalid_string: String },

    /// Failed to parse day from text.
    #[error("Invalid day from String: {invalid_string}, reason: {reason}")]
    InvalidDayFromString {
        invalid_string: String,
        reason: String,
    },

    /// Day was outside valid bounds.
    #[error("Invalid day: {day}, reason: {reason}")]
    InvalidDay { day: i32, reason: String },

    /// Month was outside valid bounds.
    #[error("Invalid month: {month}, reason: {reason}")]
    InvalidMonth { month: i32, reason: String },

    /// Failed to parse month from text.
    #[error("Failed to parse Month: {invalid_string}")]
    FailedToParseMonth { invalid_string: String },

    /// Failed to parse date text.
    #[error("Failed to parse date, because: {underlying}")]
    FailedToParseDate { underlying: String },

    /// Failed to parse a hex color string.
    #[error("Invalid hex color format: {invalid_string}")]
    InvalidHexColor { invalid_string: String },

    /// Date components were invalid.
    #[error("Invalid date, underlying: {underlying}")]
    InvalidDate { underlying: String },
}
