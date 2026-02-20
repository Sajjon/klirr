use crate::{Day, Error, HasSample, Month, PaymentTerms, Result, Year};
use bon::Builder;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use derive_more::Display;
use getset::Getters;
use serde_with::DeserializeFromStr;
use serde_with::SerializeDisplay;

/// A date relevant for the invoice, e.g. invoice date, due date or a transaction
/// date for an expense.
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    Builder,
    Getters,
)]
#[display("{year:04}-{month:02}-{day:02}")]
pub struct Date {
    /// e.g. 2025
    #[builder(into)]
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
    type Err = crate::Error;

    /// Parses a date from one of the supported formats:
    /// - `YYYY-MM-DD`
    /// - `YYYY-MM` (interpreted as month-end)
    /// - `YYYY-MM-first-half` / `YYYY-MM-second-half`
    /// # Errors
    /// Returns an error if the string is not in the correct format or if the
    /// year, month, or day is invalid.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let date: Date = "2025-05-23".parse().unwrap();
    /// assert_eq!(date.year(), &Year::from(2025));
    /// assert_eq!(date.month(), &Month::May);
    /// assert_eq!(date.day(), &Day::try_from(23).unwrap());
    ///
    /// let month_end: Date = "2025-05".parse().unwrap();
    /// assert_eq!(month_end.to_string(), "2025-05-31");
    ///
    /// let first_half: Date = "2025-02-first-half".parse().unwrap();
    /// assert_eq!(first_half.to_string(), "2025-02-14");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, '-');
        let Some(year_part) = parts.next() else {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format".to_owned(),
            });
        };
        let Some(month_part) = parts.next() else {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format".to_owned(),
            });
        };

        let year = Year::from_str(year_part)?;
        let month = Month::from_str(month_part)?;

        match parts.next() {
            None => {
                let day = month.last_day(year);
                Ok(Self::builder().year(year).month(month).day(day).build())
            }
            Some(day_or_half) => {
                if let Ok(day) = Day::from_str(day_or_half) {
                    return Ok(Self::builder().year(year).month(month).day(day).build());
                }

                let day = match day_or_half {
                    "first" | "first-half" | "1" => {
                        if month == Month::February {
                            Day::try_from(14).expect("valid day")
                        } else {
                            Day::try_from(15).expect("valid day")
                        }
                    }
                    "second" | "second-half" | "2" => month.last_day(year),
                    _ => {
                        return Err(Error::FailedToParseDate {
                            underlying: "Invalid Format".to_owned(),
                        });
                    }
                };

                Ok(Self::builder().year(year).month(month).day(day).build())
            }
        }
    }
}

fn from_ymd_parts(year: i32, month: u32, day: u32) -> Date {
    Date::builder()
        .year(year)
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
    /// Creates a date from year-month-day components.
    pub fn from_ymd(
        year: impl Into<i32>,
        month: impl Into<u32>,
        day: impl Into<u32>,
    ) -> Result<Self> {
        let year = year.into();
        let month = month.into();
        let day = day.into();
        let naive = NaiveDate::from_ymd_opt(year, month, day).ok_or(Error::InvalidDate {
            underlying: format!("invalid Y-M-D: {year}-{month}-{day}"),
        })?;
        Ok(Self::from(naive))
    }

    /// Returns today's local date.
    pub fn today() -> Self {
        Self::from(Local::now().date_naive())
    }

    /// Returns the last day in this date's month.
    pub fn last_day_of_month(&self) -> Day {
        self.month().last_day(*self.year())
    }

    /// Returns a date set to this date's month-end.
    pub fn end_of_month(&self) -> Self {
        Self::builder()
            .year(*self.year())
            .month(*self.month())
            .day(self.last_day_of_month())
            .build()
    }

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
            .year(2025)
            .month(Month::May)
            .day(Day::try_from(31).expect("LEQ 31 days"))
            .build()
    }
    fn sample_other() -> Self {
        Self::builder()
            .year(2024)
            .month(Month::December)
            .day(Day::try_from(15).expect("LEQ 31 days"))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use std::str::FromStr;
    use test_log::test;

    type Sut = Date;

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
    fn test_date_from_str() {
        let sut = Sut::from_str("2025-05-23").unwrap();
        assert_eq!(sut.year(), &Year::from(2025));
        assert_eq!(sut.month(), &Month::May);
        assert_eq!(sut.day(), &Day::try_from(23).unwrap());
    }

    #[test]
    fn test_from_str_all_reasons_invalid() {
        let invalid_dates = [
            "2025-05-32",        // Invalid day
            "99999999999-05-32", // Invalid year
            "2025-13-01",        // Invalid month
            "2025-00-01",        // Invalid month zero
            "2025-13-01",        // Invalid month too large
            "2025",              // Missing month and day
            "05-23",             // Missing year
            "2025-05-23-01",     // Too many parts
        ];

        for date in invalid_dates {
            assert!(Sut::from_str(date).is_err());
        }
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
