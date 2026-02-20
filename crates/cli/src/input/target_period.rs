use crate::{Cadence, HasSample, PeriodAnno, YearAndMonth, YearMonthAndFortnight};
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
    pub fn period_for_cadence(&self, cadence: Cadence) -> PeriodAnno {
        match cadence {
            Cadence::Monthly => match self {
                Self::Current => YearAndMonth::current().into(),
                Self::Last => YearAndMonth::last().into(),
            },
            Cadence::BiWeekly => match self {
                Self::Current => YearMonthAndFortnight::current().into(),
                Self::Last => YearMonthAndFortnight::last().into(),
            },
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
    fn period_for_cadence_monthly_current() {
        let target = Sut::Current;
        let period = target.period_for_cadence(Cadence::Monthly);
        assert_eq!(period, YearAndMonth::current().into());
    }

    #[test]
    fn period_for_cadence_monthly_last() {
        let target = Sut::Last;
        let period = target.period_for_cadence(Cadence::Monthly);
        assert_eq!(period, YearAndMonth::last().into());
    }

    #[test]
    fn period_for_cadence_biweekly_current() {
        let target = Sut::Current;
        let period = target.period_for_cadence(Cadence::BiWeekly);
        assert_eq!(period, YearMonthAndFortnight::current().into());
    }

    #[test]
    fn period_for_cadence_biweekly_last() {
        let target = Sut::Last;
        let period = target.period_for_cadence(Cadence::BiWeekly);
        assert_eq!(
            period,
            YearMonthAndFortnight::current().one_half_earlier().into()
        );
    }
}
