use crate::prelude::*;

/// A date relevant for the invoice, e.g. invoice date, due date or a transaction
/// date for an expense.
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    Builder,
    Getters,
)]
#[display("{year:04}-{month:02}-{day:02}")]
pub struct Date {
    /// e.g. 2025
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[getset(get = "pub")]
    month: Month,

    /// e.g. 31 for the last day of May
    #[getset(get = "pub")]
    day: Day,
}

impl std::str::FromStr for Date {
    type Err = crate::prelude::Error;

    /// Parses a date in the format "YYYY-MM-DD", e.g. "2025-05-23".
    /// # Errors
    /// Returns an error if the string is not in the correct format or if the
    /// year, month, or day is invalid.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let date: Date = "2025-05-23".parse().unwrap();
    /// assert_eq!(date.year(), &Year::from(2025));
    /// assert_eq!(date.month(), &Month::May);
    /// assert_eq!(date.day(), &Day::try_from(23).unwrap());    
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format".to_owned(),
            });
        }

        let year = Year::from_str(parts[0])?;
        let month = Month::from_str(parts[1])?;
        let day = Day::from_str(parts[2])?;

        Ok(Self::builder().year(year).month(month).day(day).build())
    }
}

fn from_ymd_parts(year: i32, month: u32, day: u32) -> Date {
    Date::builder()
        .year(year.into())
        .month(Month::try_from(month).expect("Invalid month"))
        .day(Day::try_from(day).expect("Invalid day"))
        .build()
}

impl From<NaiveDate> for Date {
    fn from(value: NaiveDate) -> Self {
        from_ymd_parts(value.year(), value.month(), value.day())
    }
}

impl From<NaiveDateTime> for Date {
    fn from(value: NaiveDateTime) -> Self {
        from_ymd_parts(value.year(), value.month(), value.day())
    }
}

impl Date {
    pub fn to_datetime(&self) -> NaiveDateTime {
        let naive_date = chrono::NaiveDate::from_ymd_opt(
            **self.year() as i32,
            **self.month() as u32,
            **self.day() as u32,
        )
        .expect("Invalid date components");
        naive_date
            .and_hms_opt(0, 0, 0)
            .expect("Invalid time components")
    }

    pub fn advance_days(&self, days: &Day) -> Self {
        let datetime = self.to_datetime();
        let days: u8 = **days;
        let advanced_date = datetime + chrono::Duration::days(days as i64);
        Self::from(advanced_date)
    }

    pub fn advance(&self, terms: &PaymentTerms) -> Self {
        match terms {
            PaymentTerms::Net(days) => self.advance_days(days.due_in()),
        }
    }
}

impl HasSample for Date {
    fn sample() -> Self {
        Self::builder()
            .year(2025.into())
            .month(Month::May)
            .day(Day::try_from(31).expect("LEQ 31 days"))
            .build()
    }
    fn sample_other() -> Self {
        Self::builder()
            .year(2024.into())
            .month(Month::December)
            .day(Day::try_from(15).expect("LEQ 31 days"))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_date_from_str() {
        let sut = Date::from_str("2025-05-23").unwrap();
        assert_eq!(sut.year(), &Year::from(2025));
        assert_eq!(sut.month(), &Month::May);
        assert_eq!(sut.day(), &Day::try_from(23).unwrap());
    }

    #[test]
    fn test_year_month_from_str() {
        let sut = YearAndMonth::from_str("2025-05").unwrap();
        assert_eq!(sut.year(), &Year::from(2025));
        assert_eq!(sut.month(), &Month::May);
    }

    #[test]
    fn test_from_str_all_reasons_invalid() {
        let invalid_dates = [
            "2025-05-32",        // Invalid day
            "99999999999-05-32", // Invalid year
            "2025-13-01",        // Invalid month
            "2025-00-01",        // Invalid month zero
            "2025-13-01",        // Invalid month too large
            "2025-05",           // Missing day
            "2025",              // Missing month and day
            "05-23",             // Missing year
            "2025-05-23-01",     // Too many parts
        ];

        for date in invalid_dates {
            assert!(Date::from_str(date).is_err());
        }
    }

    #[test]
    fn test_compare_year_and_month() {
        let date1 = YearAndMonth::from_str("2025-05").unwrap();
        let date2 = YearAndMonth::from_str("2025-06").unwrap();
        let date3 = YearAndMonth::from_str("2024-12").unwrap();

        assert!(date1 < date2);
        assert!(date2 > date1);
        assert!(date1 > date3);
        assert!(date3 < date1);
    }

    #[test]
    fn test_from_naive_date() {
        let naive_date = NaiveDate::from_ymd_opt(2025, 5, 23).unwrap();
        let date: Date = naive_date.into();
        assert_eq!(date.year(), &Year::from(2025));
        assert_eq!(date.month(), &Month::May);
        assert_eq!(date.day(), &Day::try_from(23).unwrap());
    }
}
