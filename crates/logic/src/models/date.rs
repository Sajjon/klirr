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
    SerializeDisplay,
    DeserializeFromStr,
    TypedBuilder,
    Getters,
)]
#[display("{year:04}-{month:02}-{day:02}")]
pub struct Date {
    /// e.g. 2025
    #[builder(setter(into))]
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    month: Month,

    /// e.g. 31 for the last day of May
    #[builder(setter(into))]
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
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
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

        let year = Year::from(
            parts[0]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid year".to_owned(),
                })?,
        );
        let month =
            Month::try_from(
                parts[1]
                    .parse::<i32>()
                    .map_err(|_| Error::FailedToParseDate {
                        underlying: "Invalid month".to_owned(),
                    })?,
            )?;
        let day =
            Day::try_from(
                parts[2]
                    .parse::<i32>()
                    .map_err(|_| Error::FailedToParseDate {
                        underlying: "Invalid day".to_owned(),
                    })?,
            )?;

        Ok(Self::builder().year(year).month(month).day(day).build())
    }
}

impl From<NaiveDateTime> for Date {
    fn from(value: NaiveDateTime) -> Self {
        Self::builder()
            .year(value.year())
            .month(
                Month::try_from(value.month())
                    .expect("NativeDateTime should always return valid month."),
            )
            .day(
                Day::try_from(value.day()).expect("NativeDateTime should always return valid day."),
            )
            .build()
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

impl Date {
    pub fn sample() -> Self {
        Self::builder()
            .year(2025)
            .month(Month::May)
            .day(Day::try_from(31).expect("LEQ 31 days"))
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
            "2025-05-32",    // Invalid day
            "2025-13-01",    // Invalid month
            "2025-00-01",    // Invalid month
            "2025-05",       // Missing day
            "2025",          // Missing month and day
            "05-23",         // Missing year
            "2025-05-23-01", // Too many parts
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
}
