use crate::{define_item_struct, prelude::*};

/// A collection of expenses for a specific period, merging items that are the same
/// except for their quantity.
#[derive(Clone, Debug, Serialize, PartialEq, Deserialize, Default)]
#[serde(transparent)]
pub(super) struct ExpensesForPeriods(Vec<Item>);

#[derive(Hash, Eq, PartialEq, Display, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
struct QuantityIgnored;
define_item_struct!(pub, Marker, QuantityIgnored);

impl ExpensesForPeriods {
    /// Inserts a vector of items into the `ExpensesForPeriods`, merging items that are the same
    /// except for their quantity.
    pub(super) fn insert(&mut self, items: Vec<Item>) {
        self.0.extend(items);

        let mut map = IndexMap::<Marker, Quantity>::new();
        for item in &self.0 {
            let marker = Marker::builder()
                .name(item.name().clone())
                .transaction_date(*item.transaction_date())
                .unit_price(*item.unit_price())
                .currency(*item.currency())
                .quantity(QuantityIgnored)
                .build();

            map.entry(marker)
                .and_modify(|q| *q += *item.quantity())
                .or_insert(*item.quantity());
        }

        self.0.retain(|_| false);
        for (marker, quantity) in map {
            let item = Item::builder()
                .name(marker.name().clone())
                .transaction_date(*marker.transaction_date())
                .unit_price(*marker.unit_price())
                .currency(*marker.currency())
                .quantity(quantity)
                .build();
            self.0.push(item);
        }
    }

    /// Creates a new `ExpensesForPeriods` with the given items, merging items that are the same
    /// except for their quantity.
    pub(super) fn new(items: Vec<Item>) -> Self {
        let mut self_ = Self::default();
        self_.insert(items);
        self_
    }

    /// Returns the items in this month
    pub(super) fn items(&self) -> Vec<Item> {
        self.0.clone()
    }
}
