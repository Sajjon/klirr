use derive_more::Constructor;

use crate::{Date, Error, HasSample, Month, Result, Year, YearMonthAndFortnight};
use bon::Builder;
use derive_more::Display;
use getset::Getters;
use serde_with::DeserializeFromStr;
use serde_with::SerializeDisplay;

/// A date relevant for the invoice, e.g. invoice date, due date or a transaction
/// date for an expense.
#[derive(
    Clone,
    Copy,
    derive_more::Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    Builder,
    Getters,
    Constructor,
)]
#[display("{year:04}-{month:02}")]
#[debug("{year:04}-{month:02}")]
pub struct YearAndMonth {
    /// e.g. 2025
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[getset(get = "pub")]
    month: Month,
}

impl YearAndMonth {
    pub const fn deconstruct(self) -> (Year, Month) {
        (self.year, self.month)
    }
}
impl From<YearMonthAndFortnight> for YearAndMonth {
    fn from(value: YearMonthAndFortnight) -> Self {
        Self::builder()
            .year(*value.year())
            .month(*value.month())
            .build()
    }
}

impl From<Date> for YearAndMonth {
    /// Converts a `Date` to a `YearAndMonth`.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let date = Date::builder().year(2025).month(Month::May).day(Day::try_from(23).unwrap()).build();
    /// let year_and_month: YearAndMonth = date.into();
    /// assert_eq!(year_and_month.year(), &Year::new(2025));
    /// assert_eq!(year_and_month.month(), &Month::May);
    /// ```
    fn from(date: Date) -> Self {
        Self::builder()
            .year(*date.year())
            .month(*date.month())
            .build()
    }
}

impl YearAndMonth {
    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let january = YearAndMonth::january(2025);
    /// assert_eq!(*january.month(), Month::January);
    /// ```
    pub const fn january(year: u16) -> Self {
        Self::new(Year::new(year), Month::January)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let february = YearAndMonth::february(2025);
    /// assert_eq!(*february.month(), Month::February);
    /// ```
    pub const fn february(year: u16) -> Self {
        Self::new(Year::new(year), Month::February)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let march = YearAndMonth::march(2025);
    /// assert_eq!(*march.month(), Month::March);
    /// ```
    pub const fn march(year: u16) -> Self {
        Self::new(Year::new(year), Month::March)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;    
    /// let april = YearAndMonth::april(2025);
    /// assert_eq!(*april.month(), Month::April);
    /// ```
    pub const fn april(year: u16) -> Self {
        Self::new(Year::new(year), Month::April)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let may = YearAndMonth::may(2025);
    /// assert_eq!(*may.month(), Month::May);
    /// ```
    pub const fn may(year: u16) -> Self {
        Self::new(Year::new(year), Month::May)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let june = YearAndMonth::june(2025);
    /// assert_eq!(*june.month(), Month::June);
    /// ```
    pub const fn june(year: u16) -> Self {
        Self::new(Year::new(year), Month::June)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let july = YearAndMonth::july(2025);
    /// assert_eq!(*july.month(), Month::July);
    /// ```
    pub const fn july(year: u16) -> Self {
        Self::new(Year::new(year), Month::July)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let august = YearAndMonth::august(2025);
    /// assert_eq!(*august.month(), Month::August);
    /// ```
    pub const fn august(year: u16) -> Self {
        Self::new(Year::new(year), Month::August)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let september = YearAndMonth::september(2025);
    /// assert_eq!(*september.month(), Month::September);
    /// ```
    pub const fn september(year: u16) -> Self {
        Self::new(Year::new(year), Month::September)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let october = YearAndMonth::october(2025);
    /// assert_eq!(*october.month(), Month::October);
    /// ```
    pub const fn october(year: u16) -> Self {
        Self::new(Year::new(year), Month::October)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let november = YearAndMonth::november(2025);
    /// assert_eq!(*november.month(), Month::November);
    /// ```
    pub const fn november(year: u16) -> Self {
        Self::new(Year::new(year), Month::November)
    }

    /// Creates a new `YearAndMonth` with the given year and month.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let december = YearAndMonth::december(2025);
    /// assert_eq!(*december.month(), Month::December);
    /// ```
    pub const fn december(year: u16) -> Self {
        Self::new(Year::new(year), Month::December)
    }
}

impl HasSample for YearAndMonth {
    fn sample() -> Self {
        Self::may(2025)
    }

    fn sample_other() -> Self {
        Self::january(2024)
    }
}

impl PartialOrd for YearAndMonth {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.year > other.year {
            Some(std::cmp::Ordering::Greater)
        } else if self.year < other.year {
            Some(std::cmp::Ordering::Less)
        } else if self.month > other.month {
            Some(std::cmp::Ordering::Greater)
        } else if self.month < other.month {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl std::str::FromStr for YearAndMonth {
    type Err = crate::Error;

    /// Parses a `YearAndMonth` from a string in the format "YYYY-MM".
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let year_and_month: YearAndMonth = "2025-05".parse().unwrap();
    /// assert_eq!(year_and_month.year(), &Year::new(2025));
    /// assert_eq!(year_and_month.month(), &Month::May);
    /// ```
    ///
    /// # Errors
    /// Returns an error if the string is not in the correct format or if the year or month
    /// cannot be parsed.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format YearAndMonth".to_owned(),
            });
        }

        let year = Year::from_str(parts[0])?;
        let month = Month::from_str(parts[1])?;

        Ok(Self::builder().year(year).month(month).build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;
    use crate::HasSample;
    use insta::assert_debug_snapshot;
    use std::str::FromStr;
    use test_log::test;

    type Sut = YearAndMonth;

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
    fn test_january() {
        let january = Sut::january(2025);
        assert_eq!(january.year(), &Year::new(2025));
        assert_eq!(january.month(), &Month::January);
    }

    #[test]
    fn test_february() {
        let february = Sut::february(2025);
        assert_eq!(february.year(), &Year::new(2025));
        assert_eq!(february.month(), &Month::February);
    }

    #[test]
    fn test_march() {
        let march = Sut::march(2025);
        assert_eq!(march.year(), &Year::new(2025));
        assert_eq!(march.month(), &Month::March);
    }

    #[test]
    fn test_april() {
        let april = Sut::april(2025);
        assert_eq!(april.year(), &Year::new(2025));
        assert_eq!(april.month(), &Month::April);
    }

    #[test]
    fn test_may() {
        let may = Sut::may(2025);
        assert_eq!(may.year(), &Year::new(2025));
        assert_eq!(may.month(), &Month::May);
    }

    #[test]
    fn test_june() {
        let june = Sut::june(2025);
        assert_eq!(june.year(), &Year::new(2025));
        assert_eq!(june.month(), &Month::June);
    }

    #[test]
    fn test_july() {
        let july = Sut::july(2025);
        assert_eq!(july.year(), &Year::new(2025));
        assert_eq!(july.month(), &Month::July);
    }

    #[test]
    fn test_august() {
        let august = Sut::august(2025);
        assert_eq!(august.year(), &Year::new(2025));
        assert_eq!(august.month(), &Month::August);
    }

    #[test]
    fn test_september() {
        let september = Sut::september(2025);
        assert_eq!(september.year(), &Year::new(2025));
        assert_eq!(september.month(), &Month::September);
    }

    #[test]
    fn test_october() {
        let october = Sut::october(2025);
        assert_eq!(october.year(), &Year::new(2025));
        assert_eq!(october.month(), &Month::October);
    }
    #[test]
    fn test_november() {
        let november = Sut::november(2025);
        assert_eq!(november.year(), &Year::new(2025));
        assert_eq!(november.month(), &Month::November);
    }

    #[test]
    fn test_december() {
        let december = Sut::december(2025);
        assert_eq!(december.year(), &Year::new(2025));
        assert_eq!(december.month(), &Month::December);
    }

    #[test]
    fn test_from_str_valid() {
        let year_and_month: Sut = "2025-05".parse().unwrap();
        assert_eq!(year_and_month.year(), &Year::new(2025));
        assert_eq!(year_and_month.month(), &Month::May);
    }

    #[test]
    fn test_from_str_invalid_format() {
        let result: Result<Sut, _> = "2025/05".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_debug() {
        assert_debug_snapshot!(Sut::new(Year::new(2025), Month::May), @"2025-05");
    }

    #[test]
    fn test_compare_year_and_month() {
        let date1 = Sut::from_str("2025-05").unwrap();
        let date2 = Sut::from_str("2025-06").unwrap();
        let date3 = Sut::from_str("2024-12").unwrap();

        assert!(date1 < date2);
        assert!(date2 > date1);
        assert!(date1 > date3);
        assert!(date3 < date1);
    }

    #[test]
    fn test_from_date() {
        let date = Date::builder()
            .year(2025)
            .month(Month::May)
            .day(Day::try_from(23).unwrap())
            .build();
        let year_and_month: Sut = date.into();
        assert_eq!(year_and_month.year(), &Year::new(2025));
        assert_eq!(year_and_month.month(), &Month::May);
    }

    #[test]
    fn deconstruct() {
        let year_and_month = Sut::new(Year::new(2025), Month::May);
        let (year, month) = year_and_month.deconstruct();
        assert_eq!(year, Year::new(2025));
        assert_eq!(month, Month::May);
    }
}
