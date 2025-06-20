use crate::prelude::*;

/// Trait for types that can determine if they are expenses.
pub trait MaybeIsExpenses {
    /// Returns true if this invoice is for expenses only.
    fn is_expenses(&self) -> bool;
}

impl MaybeIsExpenses for LineItemsPricedInSourceCurrency {
    fn is_expenses(&self) -> bool {
        self.is_expenses()
    }
}

/// Services or expenses included in this invoice to be paid by the client.
#[derive(Clone, Debug, Serialize, Deserialize, From, TryUnwrap, IsVariant)]
#[from(Vec<Item>, Item)]
pub enum LineItemsPricedInSourceCurrency {
    /// Service sold by the vendor to the client, e.g. `"App development"`
    Service(Item),
    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat
    Expenses(Vec<Item>),
}

impl HasSample for LineItemsPricedInSourceCurrency {
    fn sample() -> Self {
        Self::Service(Item::sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn is_expenses() {
        assert!(!LineItemsPricedInSourceCurrency::sample().is_expenses());
    }
}
