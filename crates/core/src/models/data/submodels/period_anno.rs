use std::borrow::Borrow;

use crate::prelude::*;

/// A tagged union of period kinds.
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Hash, From, TryUnwrap,
)]
#[serde(untagged)]
pub enum PeriodAnno {
    YearAndMonth(YearAndMonth),
    YearMonthAndFortnight(YearMonthAndFortnight),
}

impl HasSample for PeriodAnno {
    fn sample() -> Self {
        YearAndMonth::sample().into()
    }

    fn sample_other() -> Self {
        YearAndMonth::sample_other().into()
    }
}
impl TryFromPeriodAnno for PeriodAnno {
    fn try_from_period_anno(period: PeriodAnno) -> Result<Self> {
        Ok(period)
    }
}
impl IsPeriod for PeriodAnno {
    fn max_granularity(&self) -> Granularity {
        match self {
            Self::YearAndMonth(period) => period.max_granularity(),
            Self::YearMonthAndFortnight(period) => period.max_granularity(),
        }
    }

    fn elapsed_periods_since(&self, start: impl Borrow<Self>) -> u16 {
        match (self, start.borrow()) {
            (Self::YearAndMonth(lhs), Self::YearAndMonth(rhs)) => lhs.elapsed_periods_since(rhs),
            (Self::YearMonthAndFortnight(lhs), Self::YearMonthAndFortnight(rhs)) => {
                lhs.elapsed_periods_since(rhs)
            }
            _ => panic!("Cannot mix period kinds"),
        }
    }

    fn to_date_end_of_period(&self) -> Date {
        match self {
            Self::YearAndMonth(period) => period.to_date_end_of_period(),
            Self::YearMonthAndFortnight(period) => period.to_date_end_of_period(),
        }
    }

    fn year(&self) -> &Year {
        match self {
            Self::YearAndMonth(period) => period.year(),
            Self::YearMonthAndFortnight(period) => period.year(),
        }
    }

    fn month(&self) -> &Month {
        match self {
            Self::YearAndMonth(period) => period.month(),
            Self::YearMonthAndFortnight(period) => period.month(),
        }
    }
}

#[cfg(test)]
mod tests {

    use insta::assert_ron_snapshot;

    use super::*;

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
        assert_eq!(late.elapsed_periods_since(&early), 2);
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
        assert_eq!(late.elapsed_periods_since(&early), 3); // whole of january (2) + half of december (1)
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
}
