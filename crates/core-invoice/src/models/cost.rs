use crate::{Decimal, HasSample};
use derive_more::Deref;
use derive_more::Display;
use derive_more::From;
use rust_decimal::dec;
use serde::Deserialize;
use serde::Serialize;

/// The total cost of an item, e.g. the total cost of a consulting service.
/// Being the quantity multiplied by the unit price.
#[derive(
    Clone, Copy, Display, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, From, Deref,
)]
#[from(forward)]
#[deref(forward)]
pub struct Cost(Decimal);

impl HasSample for Cost {
    fn sample() -> Self {
        Self::from(dec!(350.0))
    }
    fn sample_other() -> Self {
        Self::from(dec!(500.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;

    type Sut = Cost;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
