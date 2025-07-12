use crate::prelude::*;

#[derive(Debug, Clone, Copy, Display, Default, PartialEq, Parser, FromStr)]
#[command(name = "invoice")]
#[command(about = "Generate an invoice PDF", long_about = None)]
pub enum TargetMonth {
    Current,
    #[default]
    Last,
}

impl TargetMonth {
    pub fn year_and_month(&self) -> YearAndMonth {
        match self {
            TargetMonth::Current => YearAndMonth::current(),
            TargetMonth::Last => YearAndMonth::last(),
        }
    }
}

impl HasSample for TargetMonth {
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

    type Sut = TargetMonth;

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
        let target = TargetMonth::Current;
        let year_and_month = target.year_and_month();
        assert_eq!(year_and_month, YearAndMonth::current());
    }

    #[test]
    fn target_month_last() {
        let target = TargetMonth::Last;
        let year_and_month = target.year_and_month();
        assert_eq!(year_and_month, YearAndMonth::current().one_month_earlier());
    }
}
