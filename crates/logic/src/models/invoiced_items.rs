use crate::prelude::*;

/// The items being invoiced this month, either services or expenses.
#[derive(Clone, Debug, Display, Serialize, Deserialize, IsVariant, PartialEq)]
pub enum InvoicedItems {
    #[display("Service {{ days_off: {} }} ", days_off.map(|d| *d).unwrap_or(0))]
    Service { days_off: Option<Day> },
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
        Self::Service { days_off: None }
    }
}

impl HasSample for InvoicedItems {
    fn sample() -> Self {
        Self::Service {
            days_off: Some(Day::sample()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn is_expenses() {
        assert!(!InvoicedItems::Service { days_off: None }.is_expenses());
        assert!(InvoicedItems::Expenses.is_expenses());
    }
}
