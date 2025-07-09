use crate::prelude::*;

/// Records periods of unpaid leave, each period spanning at least one month.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, From, Deref, Default)]
#[from(IndexSet<YearAndMonth>)]
pub struct MonthsOffRecord(IndexSet<YearAndMonth>);

impl MonthsOffRecord {
    /// Creates a new `MonthsOffRecord` from an iterator of `YearAndMonth`.
    pub fn new(periods: impl IntoIterator<Item = YearAndMonth>) -> Self {
        Self(IndexSet::from_iter(periods))
    }

    /// Inserts a new month off period into the record of months off.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let mut record = MonthsOffRecord::new([
    ///    YearAndMonth::may(2025),
    /// ]);
    /// record.insert(
    ///    YearAndMonth::june(2025),
    /// );
    /// assert!(record.contains(&YearAndMonth::june(2025)));
    /// ```
    pub fn insert(&mut self, year_and_month: YearAndMonth) {
        self.0.insert(year_and_month);
    }

    /// Checks if this record contains a specific `YearAndMonth`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    ///
    /// let periods = [
    ///     YearAndMonth::builder().year(2023.into()).month(Month::January).build(),
    ///     YearAndMonth::builder().year(2024.into()).month(Month::May).build(),
    /// ];
    /// let record = MonthsOffRecord::new(periods);
    ///
    /// let included = YearAndMonth::builder().year(2024.into()).month(Month::May).build();
    /// let not_included = YearAndMonth::builder().year(2022.into()).month(Month::October).build();
    ///
    /// assert!(record.contains(&included));
    /// assert!(!record.contains(&not_included));
    /// ```
    pub fn contains(&self, year_and_month: &YearAndMonth) -> bool {
        self.0.contains(year_and_month)
    }
}

impl HasSample for MonthsOffRecord {
    fn sample() -> Self {
        Self::new([
            YearAndMonth::builder()
                .year(2020.into())
                .month(Month::December)
                .build(),
            YearAndMonth::builder()
                .year(2021.into())
                .month(Month::March)
                .build(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_months_off_record_insert_and_contains() {
        let mut record = MonthsOffRecord::sample();
        let new_month = YearAndMonth::june(2025);
        record.insert(new_month);
        assert!(record.contains(&new_month));
    }
}
