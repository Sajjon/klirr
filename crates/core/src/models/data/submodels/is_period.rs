use std::{borrow::Borrow, hash::Hash};

use crate::prelude::*;

/// Trait for types that can be converted from `PeriodAnno`.
pub trait TryFromPeriodAnno: Sized {
    fn try_from_period_anno(period: PeriodAnno) -> Result<Self>;
}

/// Trait for types that can be used as periods off in a record.
pub trait PeriodMarker:
    Eq + PartialOrd + Hash + Clone + std::fmt::Debug + Into<PeriodAnno> + TryFromPeriodAnno
{
}

impl<T: Eq + PartialOrd + Hash + Clone + std::fmt::Debug + Into<PeriodAnno> + TryFromPeriodAnno>
    PeriodMarker for T
{
}

pub trait IsPeriod: PeriodMarker {
    fn max_granularity(&self) -> Granularity;
    fn elapsed_periods_since(&self, start: impl Borrow<Self>) -> Result<u16>;
    fn to_date_end_of_period(&self) -> Date;
    fn year(&self) -> &Year;
    fn month(&self) -> &Month;
}
