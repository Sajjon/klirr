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

impl IsPeriod for PeriodAnno {
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
}
