use derive_more::FromStr;

use crate::{Error, Granularity, HasSample, Result};
use derive_more::Display;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumIter;

/// How often you invoice, e.g. once or twice per month
#[derive(
    Clone, Copy, Debug, Display, FromStr, Default, Serialize, Deserialize, PartialEq, EnumIter,
)]
pub enum Cadence {
    /// Invoicing **once** per month.
    #[default]
    Monthly,
    /// Invoicing **twice** per month.
    BiWeekly,
}

impl Cadence {
    pub fn max_granularity(&self) -> Granularity {
        match self {
            Self::BiWeekly => Granularity::Fortnight,
            Self::Monthly => Granularity::Month,
        }
    }
    pub fn validate(&self, granularity: impl Into<Granularity>) -> Result<()> {
        use Cadence::*;
        use Granularity::*;
        let granularity = granularity.into();
        match (self, granularity) {
            (BiWeekly, Month) => Err(Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly),
            (BiWeekly, Fortnight | Day | Hour) => Ok(()),
            (Monthly, Fortnight | Day | Hour | Month) => Ok(()),
        }
    }
}

impl HasSample for Cadence {
    fn sample() -> Self {
        Self::Monthly
    }

    fn sample_other() -> Self {
        Self::BiWeekly
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use test_log::test;

    type Sut = Cadence;

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
    fn validate_successful() {
        assert!(Sut::Monthly.validate(Granularity::Month).is_ok());
        assert!(Sut::Monthly.validate(Granularity::Fortnight).is_ok());
        assert!(Sut::Monthly.validate(Granularity::Day).is_ok());
        assert!(Sut::Monthly.validate(Granularity::Hour).is_ok());

        assert!(Sut::BiWeekly.validate(Granularity::Month).is_err());
        assert!(Sut::BiWeekly.validate(Granularity::Fortnight).is_ok());
        assert!(Sut::BiWeekly.validate(Granularity::Day).is_ok());
        assert!(Sut::BiWeekly.validate(Granularity::Hour).is_ok());
    }
}
