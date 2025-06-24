use crate::prelude::*;

/// The total cost of an item, e.g. the total cost of a consulting service.
/// Being the quantity multiplied by the unit price.
#[derive(
    Clone, Copy, Display, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, From, Deref,
)]
#[from(forward)]
#[deref(forward)]
pub struct Cost(F64);

impl HasSample for Cost {
    fn sample() -> Self {
        Self::from(350.0)
    }
}
