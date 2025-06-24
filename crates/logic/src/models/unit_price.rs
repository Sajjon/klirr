use derive_more::FromStr;

use crate::prelude::*;

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
pub struct UnitPrice(F64);

impl HasSample for UnitPrice {
    fn sample() -> Self {
        Self::from(350.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn unit_price_sample() {
        let sample = UnitPrice::sample();
        assert_eq!(*sample, 350.0);
    }

    #[test]
    fn unit_price_display() {
        let unit_price = UnitPrice::from(150.0);
        assert_eq!(format!("{}", unit_price), "150");
    }
}
