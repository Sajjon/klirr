use crate::prelude::*;

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
