use crate::{Cadence, Granularity, HasSample, RelativeTime};
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
    /// Last period, e.g. last month or last fortnight
    Last,
}

impl TargetPeriod {
    pub fn relative_time_for_cadence(&self, cadence: Cadence) -> RelativeTime {
        let unit = cadence.max_granularity();
        debug_assert!(matches!(unit, Granularity::Month | Granularity::Fortnight));
        match self {
            Self::Current => RelativeTime::current(unit),
            Self::Last => RelativeTime::last(unit),
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
    fn relative_time_for_cadence_monthly_last() {
        let target = Sut::Last;
        let relative = target.relative_time_for_cadence(Cadence::Monthly);
        assert_eq!(*relative.unit(), Granularity::Month);
        assert_eq!(*relative.amount(), -1);
    }

    #[test]
    fn relative_time_for_cadence_biweekly_current() {
        let target = Sut::Current;
        let relative = target.relative_time_for_cadence(Cadence::BiWeekly);
        assert_eq!(*relative.unit(), Granularity::Fortnight);
        assert_eq!(*relative.amount(), 0);
    }
}
