use crate::prelude::*;

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
    use test_log::test;

    #[test]
    fn item_converted_into_target_currency_sample() {
        let sample = ItemConvertedIntoTargetCurrency::sample();
        assert_eq!(*sample.total_cost(), Cost::sample());
    }
}
