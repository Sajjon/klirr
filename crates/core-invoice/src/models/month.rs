use crate::{Day, Error, FromStr, Result, Year};
use derive_more::Display;
use derive_more::IsVariant;
use serde::Deserialize;
use serde::Serialize;

/// A month of the year, e.g. 1 for January, 2 for February, etc.
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Serialize, Deserialize, IsVariant,
)]
#[display("{}", self.month())]
pub enum Month {
    /// January.
    January = 1,
    /// February.
    February,
    /// March.
    March,
    /// April.
    April,
    /// May.
    May,
    /// June.
    June,
    /// July.
    July,
    /// August.
    August,
    /// September.
    September,
    /// October.
    October,
    /// November.
    November,
    /// December.
    December,
}

impl std::fmt::Debug for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.month())
    }
}

impl Month {
    /// Returns the month as a number, e.g. 1 for January, 2 for February, etc.
    /// This is useful for serialization and comparisons.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// assert_eq!(Month::January.month(), &1);
    /// assert_eq!(Month::December.month(), &12);
    /// ```
    pub fn month(&self) -> &u8 {
        match self {
            Month::January => &1,
            Month::February => &2,
            Month::March => &3,
            Month::April => &4,
            Month::May => &5,
            Month::June => &6,
            Month::July => &7,
            Month::August => &8,
            Month::September => &9,
            Month::October => &10,
            Month::November => &11,
            Month::December => &12,
        }
    }

    /// Returns the last day in this month for the given year.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    ///
    /// assert_eq!(Month::January.last_day(Year::from(2025)), Day::try_from(31).unwrap());
    /// assert_eq!(Month::February.last_day(Year::from(2024)), Day::try_from(29).unwrap());
    /// assert_eq!(Month::February.last_day(Year::from(2025)), Day::try_from(28).unwrap());
    /// ```
    pub fn last_day(&self, year: Year) -> Day {
        match self {
            Self::January
            | Self::March
            | Self::May
            | Self::July
            | Self::August
            | Self::October
            | Self::December => Day::try_from(31).expect("31 is a valid day"),
            Self::April | Self::June | Self::September | Self::November => {
                Day::try_from(30).expect("30 is a valid day")
            }
            Self::February => {
                if year.is_leap() {
                    Day::try_from(29).expect("29 is a valid day")
                } else {
                    Day::try_from(28).expect("28 is a valid day")
                }
            }
        }
    }
}
impl std::ops::Deref for Month {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        self.month()
    }
}

impl TryFrom<i32> for Month {
    type Error = crate::Error;

    /// Attempts to convert an integer to a `Month`.
    /// The integer must be between 1 and 12, inclusive.
    /// If the integer is outside this range, an `Error::InvalidMonth` is returned
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let march = Month::try_from(3).unwrap();
    /// assert_eq!(march.to_string(), "3".to_owned());
    /// ```
    fn try_from(month: i32) -> Result<Self> {
        match month {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(Error::InvalidMonth {
                month,
                reason: "Month must be between 1 and 12".to_string(),
            }),
        }
    }
}

impl FromStr for Month {
    type Err = crate::Error;

    /// Parses a month from a string.
    /// The string must be a valid month number (1-12).
    /// If the string is not a valid month, an `Error::InvalidMonth` is returned.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let month: Month = "3".parse().unwrap();
    /// assert_eq!(month, Month::March);
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        let month = s.parse::<i32>().map_err(|_| Error::FailedToParseMonth {
            invalid_string: s.to_owned(),
        })?;
        Self::try_from(month)
    }
}

impl TryFrom<u8> for Month {
    type Error = crate::Error;
    fn try_from(month: u8) -> Result<Self> {
        Self::try_from(month as i32)
    }
}

impl TryFrom<u32> for Month {
    type Error = crate::Error;
    fn try_from(month: u32) -> Result<Self> {
        Self::try_from(month as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use test_log::test;

    #[test]
    fn test_month_conversion() {
        assert_eq!(Month::try_from(1).unwrap(), Month::January);
        assert_eq!(Month::try_from(2).unwrap(), Month::February);
        assert_eq!(Month::try_from(7).unwrap(), Month::July);
        assert_eq!(Month::try_from(8).unwrap(), Month::August);
        assert_eq!(Month::try_from(9).unwrap(), Month::September);
        assert_eq!(Month::try_from(10).unwrap(), Month::October);
        assert_eq!(Month::try_from(11).unwrap(), Month::November);
        assert_eq!(Month::try_from(12).unwrap(), Month::December);
        assert!(Month::try_from(0).is_err());
        assert!(Month::try_from(13).is_err());
    }

    #[test]
    fn test_month_display() {
        assert_eq!(Month::January.to_string(), "1");
        assert_eq!(Month::December.to_string(), "12");
    }

    #[test]
    fn test_month_deref() {
        let month: &u8 = &Month::March;
        assert_eq!(*month, 3);
    }

    #[test]
    fn test_month_debug() {
        assert_debug_snapshot!(Month::April, @"4");
    }

    #[test]
    fn test_from_str_invalid_all_reasons() {
        let invalid_months = ["0", "13", "abc", "1.5"];
        for &invalid in &invalid_months {
            let result: Result<Month> = invalid.parse();
            assert!(
                result.is_err(),
                "Expected error for invalid month '{}'",
                invalid
            );
        }
    }
}
