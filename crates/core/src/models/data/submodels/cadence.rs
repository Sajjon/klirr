use derive_more::FromStr;

use crate::prelude::*;

/// How often you invoice, e.g. once or twice per month
#[derive(Clone, Copy, Debug, Display, FromStr, Default, Serialize, Deserialize, PartialEq)]
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
            (Self::BiWeekly, Granularity::Month) => Err(Error::InvalidUtf8), // FIXME proper error variant
            (Self::BiWeekly, Granularity::Day | Granularity::Hour) => Ok(()),
            (Self::Monthly, Granularity::Day | Granularity::Hour | Granularity::Month) => Ok(()),
        }
    }
}
