use derive_more::FromStr;

use crate::prelude::*;

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
    pub fn validate(&self, granularity: impl Into<Granularity>) -> Result<()> {
        let granularity = granularity.into();
        match (self, granularity) {
            (Self::BiWeekly, Granularity::Month) => {
                Err(Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly)
            }
            (Self::BiWeekly, Granularity::Fortnight | Granularity::Day | Granularity::Hour) => {
                Ok(())
            }
            (
                Self::Monthly,
                Granularity::Fortnight | Granularity::Day | Granularity::Hour | Granularity::Month,
            ) => Ok(()),
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
    fn validate() {
        assert!(Sut::sample().validate(Granularity::Month).is_ok());
        assert!(Sut::sample().validate(Granularity::Fortnight).is_ok());
        assert!(Sut::sample().validate(Granularity::Day).is_ok());
        assert!(Sut::sample().validate(Granularity::Hour).is_ok());

        assert!(Sut::sample_other().validate(Granularity::Month).is_err());
        assert!(Sut::sample_other().validate(Granularity::Fortnight).is_ok());
        assert!(Sut::sample_other().validate(Granularity::Day).is_ok());
        assert!(Sut::sample_other().validate(Granularity::Hour).is_ok());
    }
}
