use derive_more::FromStr;

use crate::prelude::*;

/// The granularity of invoiced quantity, i.e. if you are invoicing fixed rate
/// per month, per day or per hour.
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    FromStr,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    IsVariant,
    EnumIter,
)]
pub enum Granularity {
    Hour,
    #[default]
    Day,
    Month,
}

impl Granularity {
    pub fn example_rate(&self) -> String {
        match self {
            Self::Hour => "$150",
            Self::Day => "$1,000",
            Self::Month => "$15,000",
        }
        .to_owned()
    }
}

impl HasSample for Granularity {
    fn sample() -> Self {
        Self::Month
    }

    fn sample_other() -> Self {
        Self::Day
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = Granularity;

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
    fn example_rates() {
        assert!(!Sut::Day.example_rate().is_empty());
        assert!(!Sut::Month.example_rate().is_empty());
        assert!(!Sut::Hour.example_rate().is_empty());
    }

    #[test]
    fn ord() {
        assert!(Sut::Month > Sut::Day);
        assert!(Sut::Month > Sut::Hour);
        assert!(Sut::Day < Sut::Month);

        assert!(Sut::Day > Sut::Hour);
        assert!(Sut::Hour < Sut::Day);
        assert!(Sut::Hour < Sut::Month);
    }
}
