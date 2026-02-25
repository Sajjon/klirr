use crate::{Granularity, HasSample};
use bon::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

/// Relative period movement from "now", expressed in period units.
///
/// Examples:
/// - `{ unit: Month, amount: 0 }` means current month period
/// - `{ unit: Month, amount: -1 }` means last month period
/// - `{ unit: Fortnight, amount: -1 }` means last fortnight period
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Builder, Getters)]
pub struct RelativeTime {
    #[getset(get = "pub")]
    unit: Granularity,
    #[getset(get = "pub")]
    amount: i16,
}

impl RelativeTime {
    pub fn current(unit: Granularity) -> Self {
        Self::builder().unit(unit).amount(0).build()
    }

    pub fn last(unit: Granularity) -> Self {
        Self::builder().unit(unit).amount(-1).build()
    }
}

impl HasSample for RelativeTime {
    fn sample() -> Self {
        Self::current(Granularity::Month)
    }

    fn sample_other() -> Self {
        Self::last(Granularity::Fortnight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = RelativeTime;

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
