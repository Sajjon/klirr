use std::collections::HashSet;

use chrono::NaiveDate;
use derive_more::Deref;
use derive_more::From;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::{Date, HasSample};

/// A set of public/bank holiday dates for a single country and year.
///
/// These dates are deducted from billable working days when a vendor has opted
/// to be off on bank holidays (see
/// `klirr_core_invoice::ServiceFees::off_on_bank_holidays`). An empty set means
/// "no holidays to deduct" — the safe default that preserves prior behavior.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Deref, From)]
pub struct BankHolidays(IndexSet<Date>);

impl BankHolidays {
    /// Creates a [`BankHolidays`] set from an iterator of dates.
    pub fn new(dates: impl IntoIterator<Item = Date>) -> Self {
        Self(IndexSet::from_iter(dates))
    }

    /// Returns the holidays as a [`HashSet`] of [`NaiveDate`] for O(1) lookup
    /// while iterating days in a period.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_foundation;
    /// use klirr_foundation::*;
    ///
    /// let holidays = BankHolidays::new(["2025-06-06".parse::<Date>().unwrap()]);
    /// let naive = holidays.as_naive_dates();
    /// assert_eq!(naive.len(), 1);
    /// ```
    pub fn as_naive_dates(&self) -> HashSet<NaiveDate> {
        self.0
            .iter()
            .filter_map(|date| {
                NaiveDate::from_ymd_opt(
                    **date.year() as i32,
                    **date.month() as u32,
                    **date.day() as u32,
                )
            })
            .collect()
    }
}

impl HasSample for BankHolidays {
    fn sample() -> Self {
        // Swedish national day, 2025-06-06 (a Friday).
        Self::new(["2025-06-06".parse::<Date>().expect("valid sample date")])
    }

    fn sample_other() -> Self {
        // US Independence Day, 2025-07-04 (a Friday).
        Self::new(["2025-07-04".parse::<Date>().expect("valid sample date")])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    type Sut = BankHolidays;

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
    fn default_is_empty() {
        assert!(Sut::default().is_empty());
    }

    #[test]
    fn new_deduplicates() {
        let date = "2025-06-06".parse::<Date>().unwrap();
        let holidays = Sut::new([date, date]);
        assert_eq!(holidays.len(), 1);
    }

    #[test]
    fn as_naive_dates_roundtrips() {
        let holidays = Sut::new([
            "2025-06-06".parse::<Date>().unwrap(),
            "2025-12-25".parse::<Date>().unwrap(),
        ]);
        let naive = holidays.as_naive_dates();
        assert!(naive.contains(&NaiveDate::from_ymd_opt(2025, 6, 6).unwrap()));
        assert!(naive.contains(&NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()));
    }
}
