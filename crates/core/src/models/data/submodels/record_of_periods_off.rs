use std::{borrow::Borrow, hash::Hash};

use crate::prelude::*;

/// Trait for types that can be used as periods off in a record.
pub trait PeriodMarker:
    Eq + PartialOrd + Hash + Clone + std::fmt::Debug + Into<PeriodAnno>
{
}
impl<T: Eq + PartialOrd + Hash + Clone + std::fmt::Debug + Into<PeriodAnno>> PeriodMarker for T {}
pub trait IsPeriod: PeriodMarker {
    fn elapsed_periods_since(&self, start: impl Borrow<Self>) -> u16;
    fn to_date_end_of_period(&self) -> Date;
    fn year(&self) -> &Year;
    fn month(&self) -> &Month;
}

/// A record of periods off, e.g. `2025-05-1` for the first half of May 2025.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Deref)]
pub struct RecordOfPeriodsOff<Period: IsPeriod>(IndexSet<Period>);

impl<Record: IsPeriod> Default for RecordOfPeriodsOff<Record> {
    fn default() -> Self {
        Self::new([])
    }
}

#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Hash, From, TryUnwrap,
)]
#[serde(untagged)]
pub enum PeriodAnno {
    YearAndMonth(YearAndMonth),
    YearMonthAndFortnight(YearMonthAndFortnight),
}
impl HasSample for PeriodAnno {
    fn sample() -> Self {
        YearAndMonth::sample().into()
    }
    fn sample_other() -> Self {
        YearAndMonth::sample_other().into()
    }
}

impl IsPeriod for PeriodAnno {
    fn elapsed_periods_since(&self, start: impl Borrow<Self>) -> u16 {
        match (self, start.borrow()) {
            (Self::YearAndMonth(lhs), Self::YearAndMonth(rhs)) => lhs.elapsed_periods_since(rhs),
            (Self::YearMonthAndFortnight(lhs), Self::YearMonthAndFortnight(rhs)) => {
                lhs.elapsed_periods_since(rhs)
            }
            _ => panic!("Cannot mix period kinds"),
        }
    }

    fn to_date_end_of_period(&self) -> Date {
        match self {
            Self::YearAndMonth(period) => period.to_date_end_of_period(),
            Self::YearMonthAndFortnight(period) => period.to_date_end_of_period(),
        }
    }

    fn year(&self) -> &Year {
        match self {
            Self::YearAndMonth(period) => period.year(),
            Self::YearMonthAndFortnight(period) => period.year(),
        }
    }

    fn month(&self) -> &Month {
        match self {
            Self::YearAndMonth(period) => period.month(),
            Self::YearMonthAndFortnight(period) => period.month(),
        }
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

    pub fn from(value: impl IntoIterator<Item = Period>) -> Self {
        Self(IndexSet::from_iter(value))
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
