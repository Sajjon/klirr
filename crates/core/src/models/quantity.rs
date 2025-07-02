use crate::prelude::*;

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
)]
#[from(forward)]
#[deref(forward)]
pub struct Quantity(Decimal);
impl Quantity {
    pub const ZERO: Self = Self(Decimal::ZERO);
    pub const ONE: Self = Self(Decimal::ONE);
}

impl HasSample for Quantity {
    fn sample() -> Self {
        Self::ONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

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
