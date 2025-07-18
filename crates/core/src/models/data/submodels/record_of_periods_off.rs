use crate::prelude::*;

/// A record of periods off, e.g. `2025-05-1` for the first half of May 2025.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Deref, From)]
pub struct RecordOfPeriodsOff<Period: IsPeriod>(IndexSet<Period>);

impl<Record: IsPeriod> Default for RecordOfPeriodsOff<Record> {
    fn default() -> Self {
        Self::new([])
    }
}

pub type RecordOfMonthsOff = RecordOfPeriodsOff<YearAndMonth>;
pub type RecordOfFortnightsOff = RecordOfPeriodsOff<YearMonthAndFortnight>;

pub type PeriodsOffRecord = RecordOfPeriodsOff<PeriodAnno>;

impl<Period: IsPeriod> RecordOfPeriodsOff<Period> {
    /// Creates a new `RecordOfPeriodsOff` from an iterator of `Period`.
    pub fn new(periods: impl IntoIterator<Item = Period>) -> Self {
        Self(IndexSet::from_iter(periods))
    }

    /// Inserts a new period off into the record.
    pub fn insert(&mut self, period: Period) {
        self.0.insert(period);
    }

    /// Checks if this record contains a specific period.
    pub fn contains(&self, period: &Period) -> bool {
        self.0.contains(period)
    }
}

impl<Period: IsPeriod + HasSample> HasSample for RecordOfPeriodsOff<Period> {
    fn sample() -> Self {
        Self::new([Period::sample()])
    }

    fn sample_other() -> Self {
        Self::new([Period::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = RecordOfPeriodsOff<YearAndMonth>;

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
