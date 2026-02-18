use crate::{Decimal, HasSample};
use derive_more::Deref;
use derive_more::Display;
use derive_more::From;
use rust_decimal::dec;
use serde::Deserialize;
use serde::Serialize;

/// The quantity of items, e.g. the number of days of consulting service.
#[derive(
    Clone,
    Copy,
    Display,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From,
    Deref,
    PartialOrd,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::Mul,
)]
#[from(forward)]
#[deref(forward)]
pub struct Quantity(Decimal);
impl Quantity {
    pub const ZERO: Self = Self(Decimal::ZERO);
    pub const ONE: Self = Self(Decimal::ONE);
    pub const TWO: Self = Self(Decimal::TWO);
    pub const EIGHT: Self = Self(Decimal::EIGHT);
}

impl HasSample for Quantity {
    fn sample() -> Self {
        Self::ONE
    }
    fn sample_other() -> Self {
        Self::from(dec!(3.5)) // Example of a different quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use rust_decimal::dec;
    use test_log::test;

    type Sut = Quantity;

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
    fn quantity_sample() {
        let sample = Quantity::sample();
        assert_eq!(sample, Quantity::ONE);
    }

    #[test]
    fn quantity_display() {
        let quantity = Quantity::from(dec!(3.5));
        assert_eq!(format!("{}", quantity), "3.5");
    }
}
