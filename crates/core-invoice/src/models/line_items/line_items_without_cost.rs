use crate::{HasSample, Item};
use derive_more::From;
use derive_more::IsVariant;
use derive_more::TryUnwrap;
use serde::Deserialize;
use serde::Serialize;

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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, From, TryUnwrap, IsVariant)]
#[from(Vec<Item>, Item)]
pub enum LineItemsPricedInSourceCurrency {
    /// Service sold by the vendor to the client, e.g. `"Agreed Consulting Fees"`
    Service(Item),
    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat
    Expenses(Vec<Item>),
}

impl HasSample for LineItemsPricedInSourceCurrency {
    fn sample() -> Self {
        Self::Service(Item::sample())
    }

    fn sample_other() -> Self {
        Self::Expenses(vec![Item::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use test_log::test;

    type Sut = LineItemsPricedInSourceCurrency;

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
    fn is_expenses() {
        assert!(!MaybeIsExpenses::is_expenses(
            &LineItemsPricedInSourceCurrency::sample()
        ));
    }
}
