use std::ops::Deref;

use derive_more::Display;

use crate::HasSample;

/// Smallest allowed net-payment term, in days.
const MIN_DUE_DAYS: u16 = 1;
/// Largest allowed net-payment term, in days (one year).
const MAX_DUE_DAYS: u16 = 365;

/// A number of days until a payment is due, e.g. the `35` in `Net 35`.
///
/// This is a **duration**, not a calendar day-of-month, so unlike
/// [`crate::Day`] (which is constrained to 1–31) it accepts any value in
/// `1..=365`. That makes common net terms such as `Net 30`, `Net 45`, `Net 60`
/// and `Net 90` representable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct DueDays(u16);

/// Error returned when constructing a [`DueDays`] from an invalid input — either
/// an out-of-range number or a non-numeric string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidDueDays {
    /// A number outside the supported `1..=365` range.
    OutOfRange {
        /// The offending value.
        value: i64,
    },
    /// A string that is not a valid integer, e.g. `"abc"`.
    NotANumber {
        /// The offending input.
        input: String,
    },
}

impl std::fmt::Display for InvalidDueDays {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfRange { value } => write!(
                f,
                "Invalid net payment days: {value}, must be between {MIN_DUE_DAYS} and {MAX_DUE_DAYS}."
            ),
            Self::NotANumber { input } => {
                write!(f, "Invalid net payment days: '{input}' is not a number.")
            }
        }
    }
}

impl std::error::Error for InvalidDueDays {}

impl DueDays {
    /// Constructs a [`DueDays`], validating the value is within `1..=365`.
    ///
    /// # Errors
    /// Returns [`InvalidDueDays`] if the value is out of range.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_foundation;
    /// use klirr_foundation::*;
    ///
    /// assert_eq!(*DueDays::new(35).unwrap(), 35);
    /// assert!(DueDays::new(0).is_err());
    /// assert!(DueDays::new(366).is_err());
    /// ```
    pub fn new(value: u16) -> Result<Self, InvalidDueDays> {
        Self::from_i64(value as i64)
    }

    /// Validates and constructs from a signed integer, the common path for
    /// parsing and `TryFrom` conversions.
    fn from_i64(value: i64) -> Result<Self, InvalidDueDays> {
        if (MIN_DUE_DAYS as i64..=MAX_DUE_DAYS as i64).contains(&value) {
            Ok(Self(value as u16))
        } else {
            Err(InvalidDueDays::OutOfRange { value })
        }
    }
}

impl TryFrom<u16> for DueDays {
    type Error = InvalidDueDays;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<u32> for DueDays {
    type Error = InvalidDueDays;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_i64(value as i64)
    }
}

impl TryFrom<i32> for DueDays {
    type Error = InvalidDueDays;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_i64(value as i64)
    }
}

impl std::str::FromStr for DueDays {
    type Err = InvalidDueDays;

    /// Parses a number of days, e.g. `"35"`.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_foundation;
    /// use klirr_foundation::*;
    ///
    /// let due: DueDays = "35".parse().unwrap();
    /// assert_eq!(*due, 35);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let value = trimmed
            .parse::<i64>()
            .map_err(|_| InvalidDueDays::NotANumber {
                input: trimmed.to_owned(),
            })?;
        Self::from_i64(value)
    }
}

impl Deref for DueDays {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HasSample for DueDays {
    fn sample() -> Self {
        Self(30)
    }
    fn sample_other() -> Self {
        Self(15)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use test_log::test;

    type Sut = DueDays;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn accepts_common_net_terms() {
        for days in [1u16, 30, 35, 45, 60, 90, 120, 365] {
            assert_eq!(*Sut::new(days).unwrap(), days, "{days}");
        }
    }

    #[test]
    fn rejects_out_of_range() {
        assert!(Sut::new(0).is_err());
        assert!(Sut::new(366).is_err());
        assert!(Sut::try_from(0i32).is_err());
        assert!(Sut::try_from(-30i32).is_err());
        assert!(Sut::try_from(1000u32).is_err());
    }

    #[test]
    fn from_str_valid_and_invalid() {
        assert_eq!(*Sut::from_str("35").unwrap(), 35);
        assert_eq!(*Sut::from_str("  60 ").unwrap(), 60);
        assert!(Sut::from_str("0").is_err());
        assert!(Sut::from_str("366").is_err());
        assert!(Sut::from_str("abc").is_err());
        assert!(Sut::from_str("15.5").is_err());
    }

    #[test]
    fn display_is_bare_number() {
        assert_eq!(Sut::new(35).unwrap().to_string(), "35");
    }

    #[test]
    fn out_of_range_error_mentions_range_and_value() {
        let error = Sut::new(500).unwrap_err();
        assert_eq!(error, InvalidDueDays::OutOfRange { value: 500 });
        let message = error.to_string();
        assert!(message.contains("500"));
        assert!(message.contains("365"));
    }

    #[test]
    fn non_numeric_error_preserves_input() {
        let error = Sut::from_str("abc").unwrap_err();
        assert_eq!(
            error,
            InvalidDueDays::NotANumber {
                input: "abc".to_owned()
            }
        );
        let message = error.to_string();
        assert!(message.contains("abc"));
        // Must not fabricate an out-of-range number for non-numeric input.
        assert!(!message.contains("365"));
    }
}
