use std::ops::Deref;

use crate::prelude::*;

/// Invoice rate, a fixed price per month, per day or per hour.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Rate {
    /// A fixed rate per month, invoiced monthly
    Monthly(UnitPrice),
    /// A fixed rate per fortnight, invoiced bi-weekly
    Fortnight(UnitPrice),
    /// A fixed rate per day, invoice monthly or bi-weekly
    Daily(UnitPrice),
    /// A fixed rate per hour, invoice monthly or bi-weekly
    Hourly(UnitPrice),
}

impl From<(UnitPrice, Granularity)> for Rate {
    fn from((price, granularity): (UnitPrice, Granularity)) -> Self {
        match granularity {
            Granularity::Month => Self::Monthly(price),
            Granularity::Fortnight => Self::Fortnight(price),
            Granularity::Day => Self::Daily(price),
            Granularity::Hour => Self::Hourly(price),
        }
    }
}

impl Rate {
    /// A monthly fixed rate
    pub fn monthly(rate: impl Into<UnitPrice>) -> Self {
        Self::Monthly(rate.into())
    }

    /// A fortnight fixed rate
    pub fn fortnight(rate: impl Into<UnitPrice>) -> Self {
        Self::Fortnight(rate.into())
    }

    /// A daily fixed rate
    pub fn daily(rate: impl Into<UnitPrice>) -> Self {
        Self::Daily(rate.into())
    }

    /// An hourly fixed rate
    pub fn hourly(rate: impl Into<UnitPrice>) -> Self {
        Self::Hourly(rate.into())
    }

    /// Discriminator
    pub fn granularity(&self) -> Granularity {
        match self {
            Self::Monthly(_) => Granularity::Month,
            Self::Fortnight(_) => Granularity::Fortnight,
            Self::Daily(_) => Granularity::Day,
            Self::Hourly(_) => Granularity::Hour,
        }
    }

    /// The inner unit price
    pub fn unit_price(&self) -> UnitPrice {
        *self.deref()
    }
}

impl std::ops::Deref for Rate {
    type Target = UnitPrice;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Monthly(price) => price,
            Self::Fortnight(price) => price,
            Self::Daily(price) => price,
            Self::Hourly(price) => price,
        }
    }
}

impl HasSample for Rate {
    fn sample() -> Self {
        Self::Monthly(UnitPrice::sample())
    }

    fn sample_other() -> Self {
        Self::Daily(UnitPrice::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = Rate;

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
    fn unit_price() {
        assert_eq!(**Sut::hourly(150), dec!(150));
        assert_eq!(**Sut::daily(1_000), dec!(1_000));
        assert_eq!(**Sut::fortnight(9_000), dec!(9_000));
        assert_eq!(**Sut::monthly(15_000), dec!(15_000));
    }

    #[test]
    fn granularity() {
        assert_eq!(Sut::hourly(150).granularity(), Granularity::Hour);
        assert_eq!(Sut::daily(1_000).granularity(), Granularity::Day);
        assert_eq!(Sut::fortnight(9_000).granularity(), Granularity::Fortnight);
        assert_eq!(Sut::monthly(15_000).granularity(), Granularity::Month);
    }

    #[test]
    fn from_tuple() {
        assert_eq!(
            Sut::from((UnitPrice::sample(), Granularity::Month)),
            Sut::monthly(UnitPrice::sample())
        );
        assert_eq!(
            Sut::from((UnitPrice::sample(), Granularity::Day)),
            Sut::daily(UnitPrice::sample())
        );
        assert_eq!(
            Sut::from((UnitPrice::sample(), Granularity::Hour)),
            Sut::hourly(UnitPrice::sample())
        );
        assert_eq!(
            Sut::from((UnitPrice::sample(), Granularity::Fortnight)),
            Sut::fortnight(UnitPrice::sample())
        );
    }
}
