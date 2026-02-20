use derive_more::Constructor;

use crate::{
    Date, Day, Error, FromStr, Granularity, HasSample, IsPeriod, Month, MonthHalf, Result, Year,
    YearAndMonth,
};
use bon::Builder;
use derive_more::Display;
use getset::Getters;
use serde_with::DeserializeFromStr;
use serde_with::SerializeDisplay;

/// A year-month-fortnight representation, used to model a two weeks period
/// within a certain month.
///
/// This period is useful for representing bi-weekly pay periods or similar use cases.
#[derive(
    Clone,
    Copy,
    derive_more::Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    Builder,
    Getters,
    Constructor,
)]
#[display("{year:04}-{month:02}-{half}")]
#[debug("{year:04}-{month:02}-{half}")]
pub struct YearMonthAndFortnight {
    /// e.g. 2025
    #[builder(into)]
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[getset(get = "pub")]
    month: Month,

    /// Either first or second half of a month
    #[getset(get = "pub")]
    half: MonthHalf,
}

impl YearMonthAndFortnight {
    pub const fn year_and_month_with_half(year_and_month: YearAndMonth, half: MonthHalf) -> Self {
        let (year, month) = year_and_month.deconstruct();
        Self { year, month, half }
    }

    fn as_year_and_month(&self) -> YearAndMonth {
        YearAndMonth::builder()
            .year(self.year)
            .month(self.month)
            .build()
    }

    /// Returns the last day of the fortnight, which is either the 14th or
    /// 15th for the first half or the last day of the month for the second half.
    fn last_day_of_half(&self) -> Day {
        match self.half {
            MonthHalf::First => Day::try_from(if self.month == Month::February {
                14
            } else {
                15
            })
            .expect("LEQ 31"),
            MonthHalf::Second => self.as_year_and_month().last_day_of_month(),
        }
    }
}

impl IsPeriod for YearMonthAndFortnight {
    fn max_granularity(&self) -> Granularity {
        Granularity::Fortnight
    }

    fn elapsed_periods_since(&self, start: impl std::borrow::Borrow<Self>) -> Result<u16> {
        let start = start.borrow();
        let start_ym = start.as_year_and_month();
        let end_ym = self.as_year_and_month();
        let elapsed = end_ym.elapsed_months_since(start_ym)?;
        let elapsed_ym = (elapsed * 2) as i16; // two halves per month
        let start_half = i16::from(*start.half());
        let end_half = i16::from(*self.half());
        let half_diff = end_half - start_half; // -1 if start is second half, +1 if end is first half, 0 if both are same half
        let elapsed = elapsed_ym + half_diff;
        if elapsed < 0 {
            return Err(Error::StartPeriodAfterEndPeriod {
                start: start.to_string(),
                end: self.to_string(),
            });
        }
        Ok(elapsed as u16)
    }

    fn to_date_end_of_period(&self) -> Date {
        Date::builder()
            .year(*self.year())
            .month(*self.month())
            .day(self.last_day_of_half())
            .build()
    }

    fn year(&self) -> &Year {
        &self.year
    }

    fn month(&self) -> &Month {
        &self.month
    }
}

impl FromStr for YearMonthAndFortnight {
    type Err = crate::Error;

    /// Parses `"YYYY-MM-first-half` into YearMonthAndFortnight with MonthHalf::First,
    /// Parses `"YYYY-MM-second-half` into YearMonthAndFortnight with MonthHalf::Second,
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() < 3 {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format YearAndMonth".to_owned(),
            });
        }

        let year = Year::from_str(parts[0])?;
        let month = Month::from_str(parts[1])?;
        let half = MonthHalf::from_str(parts[2])?;

        Ok(Self::builder().year(year).month(month).half(half).build())
    }
}

impl HasSample for YearMonthAndFortnight {
    fn sample() -> Self {
        Self::builder()
            .year(2025)
            .month(Month::July)
            .half(MonthHalf::First)
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .year(2025)
            .month(Month::July)
            .half(MonthHalf::Second)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use insta::assert_ron_snapshot;
    use std::str::FromStr;
    use test_log::test;

    type Sut = YearMonthAndFortnight;

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
    fn elapsed_periods_since_same_is_zero() {
        let sut = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::First)
            .build();

        assert_eq!(sut.elapsed_periods_since(sut).unwrap(), 0);
    }

    #[test]
    fn elapsed_periods_since_same_month_different_half_is_1() {
        let early = Sut::builder()
            .year(2025)
            .month(Month::July)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::July)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(late.elapsed_periods_since(early).unwrap(), 1);
    }

    #[test]
    fn elapsed_periods_since_across_month_one_half_is_1() {
        let early = Sut::builder()
            .year(2025)
            .month(Month::June)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::July)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early).unwrap(), 1);
    }

    #[test]
    fn elapsed_periods_since_across_month_and_year_one_half_is_1() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::January)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early).unwrap(), 1);
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_is_second_half() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::March)
            .half(MonthHalf::First)
            .build();
        let expected = 1 + 2 + 2; // second half of 2024-12 plus whole 2025-01 and whole 2025-02
        assert_eq!(late.elapsed_periods_since(early).unwrap(), expected);
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_one_period() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::January)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early).unwrap(), 1); // Second half of December only
    }

    #[test]
    fn elapsed_periods_since_one_period_only() {
        let early = Sut::builder()
            .year(2025)
            .month(Month::May)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::June)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early).unwrap(), 1); // Second half of May only
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_is_first_half() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::March)
            .half(MonthHalf::First)
            .build();
        let expected = 2 + 2 + 2; // Whole of December, January and February
        assert_eq!(late.elapsed_periods_since(early).unwrap(), expected);
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_and_late_are_second_half() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::March)
            .half(MonthHalf::Second)
            .build();
        let expected = 1 + 2 + 2 + 1; // half Dec, half March, whole Jan/Feb
        assert_eq!(late.elapsed_periods_since(early).unwrap(), expected);
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_is_first_half_and_late_is_second_half() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::March)
            .half(MonthHalf::Second)
            .build();
        let expected = 2 + 2 + 2 + 1; // whole Dec/Jan/Feb, half March
        assert_eq!(late.elapsed_periods_since(early).unwrap(), expected);
    }

    #[test]
    fn elapsed_periods_since_one_year_first_first_is_24() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early).unwrap(), 24);
    }

    #[test]
    fn elapsed_periods_since_one_year_second_second_is_24() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(late.elapsed_periods_since(early).unwrap(), 24);
    }

    #[test]
    fn elapsed_periods_since_one_year_second_first_is_23() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early).unwrap(), 23);
    }

    #[test]
    fn elapsed_periods_since_one_year_first_second_is_25() {
        let early = Sut::builder()
            .year(2024)
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025)
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(late.elapsed_periods_since(early).unwrap(), 25);
    }

    #[test]
    fn to_date_end_of_period_january_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::January)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-01-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_january_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::January)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-01-31").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_non_leap_year_february_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::February)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-02-14").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_non_leap_year_february_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::February)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-02-28").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_leap_year_february_first_half() {
        let sut = Sut::builder()
            .year(2028)
            .month(Month::February)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2028-02-14").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_leap_year_february_second_half() {
        let sut = Sut::builder()
            .year(2028)
            .month(Month::February)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2028-02-29").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_march_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::March)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-03-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_march_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::March)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-03-31").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_april_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::April)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-04-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_april_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::April)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-04-30").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_may_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::May)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-05-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_may_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::May)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-05-31").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_june_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::June)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-06-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_june_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::June)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-06-30").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_july_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::July)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-07-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_july_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::July)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-07-31").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_august_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::August)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-08-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_august_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::August)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-08-31").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_september_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::September)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-09-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_september_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::September)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-09-30").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_october_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::October)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-10-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_october_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::October)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-10-31").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_november_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::November)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-11-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_november_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::November)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-11-30").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_december_first_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-12-15").unwrap()
        );
    }

    #[test]
    fn to_date_end_of_period_december_second_half() {
        let sut = Sut::builder()
            .year(2025)
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(
            sut.to_date_end_of_period(),
            Date::from_str("2025-12-31").unwrap()
        );
    }

    #[test]
    fn year() {
        assert_eq!(
            **Sut::builder()
                .year(2025)
                .month(Month::July)
                .half(MonthHalf::First)
                .build()
                .year(),
            2025
        );
    }

    #[test]
    fn month() {
        assert_eq!(
            *Sut::builder()
                .year(2025)
                .month(Month::July)
                .half(MonthHalf::First)
                .build()
                .month(),
            Month::July
        );
    }

    #[test]
    fn from_str_valid() {
        let sut = Sut::from_str("2025-07-first-half").unwrap();
        assert_eq!(*sut.year, 2025);
        assert_eq!(sut.month, Month::July);
        assert_eq!(sut.half, MonthHalf::First);

        let sut = Sut::from_str("2025-07-second-half").unwrap();
        assert_eq!(*sut.year, 2025);
        assert_eq!(sut.month, Month::July);
        assert_eq!(sut.half, MonthHalf::Second);
    }

    #[test]
    fn from_str_invalid() {
        assert!(Sut::from_str("foobar").is_err());
        assert!(Sut::from_str("foo-bar").is_err());
        assert!(Sut::from_str("2025-07-1a").is_err());
        assert!(Sut::from_str("2025-07-3").is_err());
        assert!(Sut::from_str("2025-13-1").is_err());
    }

    #[test]
    fn serde_sample() {
        assert_ron_snapshot!(&Sut::sample())
    }

    #[test]
    fn serde_sample_other() {
        assert_ron_snapshot!(&Sut::sample_other())
    }

    #[test]
    fn deserialize_ron() {
        let s = r#""2025-05-second-half""#;
        let sut: Sut = ron::de::from_str(s).expect("Failed to deserialize RON");
        assert_eq!(*sut.year, 2025);
        assert_eq!(sut.month, Month::May);
        assert_eq!(sut.half, MonthHalf::Second);
    }
}
