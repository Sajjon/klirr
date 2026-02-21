use crate::{Date, HasSample};
use derive_more::Deref;
use derive_more::From;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

/// A record of period-end dates when no invoice was issued.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Deref, From)]
pub struct RecordOfPeriodsOff(IndexSet<Date>);

impl Default for RecordOfPeriodsOff {
    fn default() -> Self {
        Self::new([])
    }
}

impl RecordOfPeriodsOff {
    /// Creates a new `RecordOfPeriodsOff` from an iterator of dates.
    pub fn new(periods: impl IntoIterator<Item = Date>) -> Self {
        Self(IndexSet::from_iter(periods))
    }

    /// Inserts a new period-off date into the record.
    pub fn insert(&mut self, period: Date) {
        self.0.insert(period);
    }

    /// Checks if this record contains a specific period-end date.
    pub fn contains(&self, period: &Date) -> bool {
        self.0.contains(period)
    }
}

impl HasSample for RecordOfPeriodsOff {
    fn sample() -> Self {
        Self::new([Date::sample()])
    }

    fn sample_other() -> Self {
        Self::new([Date::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = RecordOfPeriodsOff;

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
