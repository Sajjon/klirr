use crate::prelude::*;
use derive_more::FromStr;

/// The cost of a single item, e.g. the cost of one day of consulting service.
#[derive(
    Clone,
    Copy,
    Display,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Serialize,
    Deserialize,
    From,
    FromStr,
    Deref,
    derive_more::Mul,
)]
#[from(forward)]
#[deref(forward)]
pub struct UnitPrice(Decimal);

impl UnitPrice {
    pub const ZERO: Self = Self(Decimal::ZERO);
    pub const ONE: Self = Self(Decimal::ONE);
}

impl HasSample for UnitPrice {
    fn sample() -> Self {
        Self::from(Decimal::from(dec!(350.0)))
    }
    fn sample_other() -> Self {
        Self::from(Decimal::from(dec!(150.0))) // Example of a different unit price
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    use test_log::test;

    type Sut = UnitPrice;

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
    fn unit_price_sample() {
        let sample = UnitPrice::sample();
        assert_eq!(*sample, dec!(350.0));
    }

    #[test]
    fn unit_price_display() {
        let unit_price = UnitPrice::from(Decimal::from(dec!(150.0)));
        assert_eq!(format!("{}", unit_price), "150.0");
    }

    #[test]
    fn test_serde() {
        assert_ron_snapshot!(UnitPrice::sample())
    }
}
