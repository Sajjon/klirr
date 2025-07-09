use derive_more::FromStr;

use crate::prelude::*;

/// The granularity of invoiced quantity, i.e. if you are invoicing fixed rate
/// per month, per day or per hour.
#[derive(Clone, Copy, Debug, Display, FromStr, Default, Serialize, Deserialize, PartialEq)]
pub enum Granularity {
    Month,
    #[default]
    Day,
    Hour,
}

impl Granularity {
    pub fn example_rate(&self) -> String {
        match self {
            Self::Hour => "$150",
            Self::Day => "$1,000",
            Self::Month => "$15,000",
        }
        .to_owned()
    }
}
