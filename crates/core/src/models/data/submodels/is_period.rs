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
