use crate::{HasSample, YearMonthAndFortnight};
use clap::Parser;
use derive_more::Display;
use derive_more::FromStr;

#[derive(Debug, Clone, Copy, Display, Default, PartialEq, Parser, FromStr)]
#[command(name = "invoice")]
#[command(about = "Generate an invoice PDF", long_about = None)]
pub enum TargetPeriod {
    /// Current period, e.g. current month or current fortnight
    Current,
    #[default]
    /// Last period, e.g. last month of last fortnight
    Last,
}

impl TargetPeriod {
    /// Note we return `YearMonthAndFortnight` since it has higher granularity than
    /// `YearAndMonth`, so we can always turn a `YearMonthAndFortnight` into a
    /// `YearAndMonth`, later in the flow if that matches the invoice cadence.
    pub fn period(&self) -> YearMonthAndFortnight {
        match self {
            Self::Current => YearMonthAndFortnight::current(),
            Self::Last => YearMonthAndFortnight::last(),
        }
    }
}

impl HasSample for TargetPeriod {
    fn sample() -> Self {
        Self::Current
    }

    fn sample_other() -> Self {
        Self::Last
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    type Sut = TargetPeriod;

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
    fn target_month_current() {
        let target = Sut::Current;
        let period = target.period();
        assert_eq!(period, YearMonthAndFortnight::current());
    }

    #[test]
    fn target_month_last() {
        let target = Sut::Last;
        let period = target.period();
        assert_eq!(period, YearMonthAndFortnight::current().one_half_earlier());
    }
}
