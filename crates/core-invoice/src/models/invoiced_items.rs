use crate::{HasSample, MaybeIsExpenses, Quantity, TimeOff};
use derive_more::Display;
use derive_more::IsVariant;
use serde::Deserialize;
use serde::Serialize;

/// The items being invoiced this month, either services or expenses.
#[derive(Clone, Debug, Display, Serialize, Deserialize, IsVariant, PartialEq)]
pub enum InvoicedItems {
    /// Service invoice mode.
    #[display("Service {{ time_off: {} }} ", time_off.map(|d| *d).unwrap_or(Quantity::ZERO))]
    Service {
        /// Optional time off deducted from the invoiced service period.
        time_off: Option<TimeOff>,
    },
    /// Expenses invoice mode.
    #[display("Expenses")]
    Expenses,
}
impl MaybeIsExpenses for InvoicedItems {
    fn is_expenses(&self) -> bool {
        self.is_expenses()
    }
}

impl Default for InvoicedItems {
    fn default() -> Self {
        Self::Service { time_off: None }
    }
}

impl HasSample for InvoicedItems {
    fn sample() -> Self {
        Self::Service {
            time_off: Some(TimeOff::sample()),
        }
    }
    fn sample_other() -> Self {
        Self::Expenses
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use test_log::test;

    type Sut = InvoicedItems;

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
        assert!(MaybeIsExpenses::is_expenses(&Sut::Expenses));
        assert!(!MaybeIsExpenses::is_expenses(&Sut::Service {
            time_off: None
        }));
    }
}
