use crate::{define_item_struct, prelude::*};

/// A collection of expenses for a specific period, merging items that are the same
/// except for their quantity.
#[derive(Clone, Debug, Serialize, PartialEq, Deserialize, Default)]
#[serde(transparent)]
pub(super) struct ExpensesForPeriods(Vec<Item>);

/// Ephemeral type used such that expensed items sharing all fields except for quantity
/// are considered the same, allowing us to merge them into a single item with the total quantity.
#[derive(Hash, Eq, PartialEq, Display, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
struct QuantityIgnored;
define_item_struct!(pub, ExpenseIdentifier, QuantityIgnored);

impl ExpensesForPeriods {
    /// Inserts a vector of items into the `ExpensesForPeriods`, merging items that are the same
    /// except for their quantity.
    pub(super) fn insert(&mut self, items: Vec<Item>) {
        self.0.extend(items);

        let mut map = IndexMap::<ExpenseIdentifier, Quantity>::new();
        for item in &self.0 {
            let identifier = ExpenseIdentifier::builder()
                .name(item.name().clone())
                .transaction_date(*item.transaction_date())
                .unit_price(*item.unit_price())
                .currency(*item.currency())
                .quantity(QuantityIgnored)
                .build();

            map.entry(identifier)
                .and_modify(|q| *q += *item.quantity())
                .or_insert(*item.quantity());
        }

        self.0.clear();

        for (identifier, quantity) in map {
            let item = Item::builder()
                .name(identifier.name().clone())
                .transaction_date(*identifier.transaction_date())
                .unit_price(*identifier.unit_price())
                .currency(*identifier.currency())
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
