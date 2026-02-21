use derive_more::Constructor;

use crate::{Error, Result};
use derive_more::Deref;
use derive_more::Display;
use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

/// Years since birth of Jesus christ, e.g. 2025
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Display,
    Serialize,
    Deserialize,
    From,
    Deref,
    Constructor,
)]
pub struct Year(u16);

impl std::str::FromStr for Year {
    type Err = crate::Error;

    /// Parses a year from a string, e.g. "2025".
    /// # Errors
    /// Returns an error if the string is not a valid year.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let year: Year = "2025".parse().unwrap();
    /// assert_eq!(*year, 2025);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u16>()
            .map_err(|_| Error::FailedToParseYear {
                invalid_string: s.to_owned(),
            })
            .map(Self)
    }
}

impl From<i32> for Year {
    /// Converts an `i32` year to a `Year`.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let year: Year = 2025.into();
    /// assert_eq!(*year, 2025);
    /// ```
    fn from(year: i32) -> Self {
        Self(year as u16)
    }
}

impl Year {
    /// Returns `true` if the year is a leap year.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    ///
    /// assert!(Year::from(2024).is_leap());
    /// assert!(!Year::from(2025).is_leap());
    /// ```
    pub fn is_leap(&self) -> bool {
        let year = **self;
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}
