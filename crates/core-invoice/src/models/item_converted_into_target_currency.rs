use crate::{Cost, HasSample, Item};
use bon::Builder;
use derive_more::Deref;
use derive_more::From;
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;

/// An item with a total cost, calculated as `unit_price * quantity`.
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Deref, From, Getters, Builder,
)]
pub struct ItemConvertedIntoTargetCurrency {
    /// An item in the currency it was paid in.
    #[deref]
    #[serde(flatten)]
    in_source_currency: Item,

    /// The total cost of the item, calculated as `unit_price * quantity`
    #[getset(get = "pub")]
    total_cost: Cost,
}

impl HasSample for ItemConvertedIntoTargetCurrency {
    fn sample() -> Self {
        Self::builder()
            .in_source_currency(Item::sample())
            .total_cost(Cost::sample())
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .in_source_currency(Item::sample_other())
            .total_cost(Cost::sample_other())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use test_log::test;

    type Sut = ItemConvertedIntoTargetCurrency;

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
    fn item_converted_into_target_currency_sample() {
        let sample = Sut::sample();
        assert_eq!(*sample.total_cost(), Cost::sample());
    }
}
