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

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

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
