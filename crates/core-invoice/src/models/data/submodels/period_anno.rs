use std::borrow::Borrow;

use crate::{
    Date, Error, Granularity, HasSample, IsPeriod, Month, Result, TryFromPeriodAnno, Year,
    YearAndMonth, YearMonthAndFortnight,
};
use derive_more::From;
use derive_more::TryUnwrap;
use serde::Deserialize;
use serde::Serialize;

/// A tagged union of period kinds.
#[derive(
    Clone,
    derive_more::Display,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    Eq,
    Hash,
    From,
    TryUnwrap,
)]
#[serde(untagged)]
pub enum PeriodAnno {
    /// A year and month, e.g. `2024-12`.
    YearAndMonth(YearAndMonth),
    /// A year, month and fortnight, e.g. `2024-12-second-half`.
    YearMonthAndFortnight(YearMonthAndFortnight),
}

impl std::str::FromStr for PeriodAnno {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(period) = YearAndMonth::from_str(s) {
            Ok(Self::YearAndMonth(period))
        } else if let Ok(period) = YearMonthAndFortnight::from_str(s) {
            Ok(Self::YearMonthAndFortnight(period))
        } else {
            Err(Error::InvalidPeriod {
                bad_value: s.to_owned(),
            })
        }
    }
}

impl HasSample for PeriodAnno {
    fn sample() -> Self {
        YearAndMonth::sample().into()
    }

    fn sample_other() -> Self {
        YearMonthAndFortnight::sample_other().into()
    }
}

impl TryFromPeriodAnno for PeriodAnno {
    fn try_from_period_anno(period: PeriodAnno) -> Result<Self> {
        Ok(period)
    }
}

impl IsPeriod for PeriodAnno {
    /// The max granularity of the period, which is the most granular period
    /// that can be represented.
    fn max_granularity(&self) -> Granularity {
        match self {
            Self::YearAndMonth(period) => period.max_granularity(),
            Self::YearMonthAndFortnight(period) => period.max_granularity(),
        }
    }

    /// Number of periods that have elapsed since the given start period.
    fn elapsed_periods_since(&self, start: impl Borrow<Self>) -> Result<u16> {
        match (self, start.borrow()) {
            (Self::YearAndMonth(lhs), Self::YearAndMonth(rhs)) => lhs.elapsed_periods_since(rhs),
            (Self::YearMonthAndFortnight(lhs), Self::YearMonthAndFortnight(rhs)) => {
                lhs.elapsed_periods_since(rhs)
            }
            (Self::YearAndMonth(_), Self::YearMonthAndFortnight(_)) => {
                Err(Error::PeriodIsNotYearAndMonth)
            }
            (Self::YearMonthAndFortnight(_), Self::YearAndMonth(_)) => {
                Err(Error::PeriodIsNotYearMonthAndFortnight)
            }
        }
    }

    /// Converts the period into a date that represents the end of the period.
    /// For `YearAndMonth`, this is the last day of the month, and for
    /// `YearMonthAndFortnight`, this is the last day of the fortnight.
    fn to_date_end_of_period(&self) -> Date {
        match self {
            Self::YearAndMonth(period) => period.to_date_end_of_period(),
            Self::YearMonthAndFortnight(period) => period.to_date_end_of_period(),
        }
    }

    /// Returns the year of the period.
    fn year(&self) -> &Year {
        match self {
            Self::YearAndMonth(period) => period.year(),
            Self::YearMonthAndFortnight(period) => period.year(),
        }
    }

    /// Returns the month of the period.
    fn month(&self) -> &Month {
        match self {
            Self::YearAndMonth(period) => period.month(),
            Self::YearMonthAndFortnight(period) => period.month(),
        }
    }
}

#[cfg(test)]
mod tests {

    use insta::{assert_ron_snapshot, assert_snapshot};

    use super::*;
    use crate::HasSample;
    use crate::MonthHalf;
    use std::str::FromStr;

    type Sut = PeriodAnno;

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
    fn test_elapsed_periods_since_year_and_month() {
        let early = Sut::YearAndMonth(YearAndMonth::december(2024));
        let late = Sut::YearAndMonth(YearAndMonth::february(2025));
        assert_eq!(late.elapsed_periods_since(&early).unwrap(), 2);
    }

    #[test]
    fn test_elapsed_periods_since_year_and_fortnight() {
        let early = Sut::YearMonthAndFortnight(
            YearMonthAndFortnight::builder()
                .year(2024.into())
                .month(Month::December)
                .half(MonthHalf::Second)
                .build(),
        );
        let late = Sut::YearMonthAndFortnight(
            YearMonthAndFortnight::builder()
                .year(2025.into())
                .month(Month::February)
                .half(MonthHalf::First)
                .build(),
        );
        assert_eq!(late.elapsed_periods_since(&early).unwrap(), 3); // whole of january (2) + half of december (1)
    }

    #[test]
    fn test_to_date_end_of_period_year_and_month() {
        let period = Sut::YearAndMonth(YearAndMonth::december(2024));
        assert_eq!(
            period.to_date_end_of_period(),
            Date::from_str("2024-12-31").unwrap()
        );
    }

    #[test]
    fn test_to_date_end_of_period_year_and_fortnight() {
        let period = Sut::YearMonthAndFortnight(
            YearMonthAndFortnight::builder()
                .year(2024.into())
                .month(Month::December)
                .half(MonthHalf::First)
                .build(),
        );
        assert_eq!(
            period.to_date_end_of_period(),
            Date::from_str("2024-12-15").unwrap()
        );
    }

    #[test]
    fn serde_fortnight() {
        assert_ron_snapshot!(Sut::YearMonthAndFortnight(
            YearMonthAndFortnight::builder()
                .year(2025.into())
                .month(Month::May)
                .half(MonthHalf::Second)
                .build()
        ));
    }

    #[test]
    fn test_deserialize_ron_year_month_and_fortnight() {
        let ron_str = r#""2025-05-second-half""#;
        let period: Sut = ron::de::from_str(ron_str).expect("Failed to deserialize RON");
        assert_eq!(
            period,
            Sut::YearMonthAndFortnight(
                YearMonthAndFortnight::builder()
                    .year(2025.into())
                    .month(Month::May)
                    .half(MonthHalf::Second)
                    .build()
            )
        );
    }

    #[test]
    fn year_month_and_fortnight_max_granularity() {
        let sut = Sut::YearMonthAndFortnight(YearMonthAndFortnight::sample());
        assert_eq!(sut.max_granularity(), Granularity::Fortnight);
    }

    #[test]
    fn mix_ym_ymf_throws() {
        let result = Sut::YearAndMonth(YearAndMonth::sample())
            .elapsed_periods_since(Sut::YearMonthAndFortnight(YearMonthAndFortnight::sample()));
        assert!(result.is_err(), "Expected error when mixing period kinds");
    }

    #[test]
    fn mix_ymf_ym_throws() {
        let result = Sut::YearMonthAndFortnight(YearMonthAndFortnight::sample())
            .elapsed_periods_since(Sut::YearAndMonth(YearAndMonth::sample()));
        assert!(result.is_err(), "Expected error when mixing period kinds");
    }

    #[test]
    fn year_month_and_fortnight_get_year() {
        let sut = Sut::YearMonthAndFortnight(
            YearMonthAndFortnight::builder()
                .year(2025.into())
                .month(Month::May)
                .half(MonthHalf::Second)
                .build(),
        );
        assert_eq!(*sut.year(), Year::from(2025));
    }

    #[test]
    fn year_month_and_fortnight_get_month() {
        let sut = Sut::YearMonthAndFortnight(
            YearMonthAndFortnight::builder()
                .year(2025.into())
                .month(Month::May)
                .half(MonthHalf::Second)
                .build(),
        );
        assert_eq!(*sut.month(), Month::May);
    }

    #[test]
    fn from_str_valid() {
        let valid_formats = vec![
            "2024-12",
            "2024-01",
            "2024-12-first-half",
            "2024-12-first",
            "2024-12-1",
            "2024-12-second-half",
            "2024-12-second",
            "2024-12-2",
        ];
        for format in valid_formats {
            let period: Sut = format.parse().expect("Failed to parse valid format");
            assert!(matches!(
                period,
                Sut::YearAndMonth(_) | Sut::YearMonthAndFortnight(_)
            ));
        }
    }

    #[test]
    fn from_str_invalid() {
        let invalid_formats = vec![
            "2024-13",             // Invalid month
            "2024-00",             // Invalid month
            "2024-11-11",          // is date, which is invalid
            "2024-12-third-half",  // Invalid fortnight
            "2024-12-third",       // Invalid fortnight
            "2024-12-3",           // Invalid fortnight
            "2024-12-fourth-half", // Invalid fortnight
            "2024-12-fourth",      // Invalid fortnight
            "2024-12-4",           // Invalid fortnight
        ];
        for format in invalid_formats {
            let result: Result<Sut, _> = format.parse();
            assert!(result.is_err(), "Expected error for format: {}", format);
        }
    }

    #[test]
    fn display_sample() {
        assert_snapshot!(Sut::sample())
    }

    #[test]
    fn display_sample_other() {
        assert_snapshot!(Sut::sample_other())
    }
}
