use crate::prelude::*;

/// Invoice rate, a fixed price per month, per day or per hour.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Rate {
    Monthly(UnitPrice),
    Daily(UnitPrice),
    Hourly(UnitPrice),
}

impl From<(UnitPrice, Granularity)> for Rate {
    fn from((price, granularity): (UnitPrice, Granularity)) -> Self {
        match granularity {
            Granularity::Month => Self::Monthly(price),
            Granularity::Day => Self::Daily(price),
            Granularity::Hour => Self::Hourly(price),
        }
    }
}

impl Rate {
    pub fn monthly(rate: impl Into<UnitPrice>) -> Self {
        Self::Monthly(rate.into())
    }
    pub fn daily(rate: impl Into<UnitPrice>) -> Self {
        Self::Daily(rate.into())
    }
    pub fn hourly(rate: impl Into<UnitPrice>) -> Self {
        Self::Hourly(rate.into())
    }

    /// Discriminator
    pub fn granularity(&self) -> Granularity {
        match self {
            Self::Monthly(_) => Granularity::Month,
            Self::Daily(_) => Granularity::Day,
            Self::Hourly(_) => Granularity::Hour,
        }
    }

    pub fn unit_price(&self) -> UnitPrice {
        match self {
            Self::Monthly(price) => *price,
            Self::Daily(price) => *price,
            Self::Hourly(price) => *price,
        }
    }
}
