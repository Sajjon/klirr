use crate::prelude::*;

/// Records periods of unpaid leave, each period spanning at least one month.
#[derive(Clone, Debug, Serialize, Deserialize, From, Deref, Default)]
#[from(IndexSet<YearAndMonth>)]
pub struct MonthsOffRecord(IndexSet<YearAndMonth>);

impl MonthsOffRecord {
    /// Creates a new `MonthsOffRecord` from an iterator of `YearAndMonth`.
    pub fn new(periods: impl IntoIterator<Item = YearAndMonth>) -> Self {
        Self(IndexSet::from_iter(periods))
    }

    /// Checks if this record contains a specific `YearAndMonth`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    ///
    /// let periods = [
    ///     YearAndMonth::builder().year(2023).month(Month::January).build(),
    ///     YearAndMonth::builder().year(2024).month(Month::May).build(),
    /// ];
    /// let record = MonthsOffRecord::new(periods);
    ///
    /// let included = YearAndMonth::builder().year(2024).month(Month::May).build();
    /// let not_included = YearAndMonth::builder().year(2022).month(Month::October).build();
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
                .year(2020)
                .month(Month::December)
                .build(),
            YearAndMonth::builder()
                .year(2021)
                .month(Month::March)
                .build(),
        ])
    }
}
