use crate::prelude::*;

/// Represents the free quantity of items, e.g. the number of days off during a
/// period, or the number of hours off during a period. Month is not part of this
/// because the most coarse grained granularity for cadence is Month, and if you
/// have been free a whole month that should be recorded in record of periods off.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum TimeOff {
    /// Amount of hours off during a period
    Hours(Quantity),
    /// Amount of days off during a period
    Days(Quantity),
}

impl TimeOff {
    pub fn granularity(&self) -> Granularity {
        match self {
            Self::Hours(_) => Granularity::Hour,
            Self::Days(_) => Granularity::Day,
        }
    }
}

impl std::ops::Deref for TimeOff {
    type Target = Quantity;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hours(q) => q,
            Self::Days(q) => q,
        }
    }
}

impl HasSample for TimeOff {
    fn sample() -> Self {
        Self::Days(Quantity::sample())
    }

    fn sample_other() -> Self {
        Self::Hours(Quantity::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = TimeOff;

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
    fn deref() {
        let hours = Sut::Hours(Quantity::sample());
        let days = Sut::Days(Quantity::sample_other());
        assert_eq!(*hours, Quantity::sample());
        assert_eq!(*days, Quantity::sample_other());
    }
}
