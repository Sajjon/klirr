use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub struct ExpensedMonths {
    explanation: String,
    #[getset(get = "pub")]
    expenses_for_months: IndexMap<YearAndMonth, Vec<Item>>,
}

impl ExpensedMonths {
    pub fn new(expenses_for_months: IndexMap<YearAndMonth, Vec<Item>>) -> Self {
        Self {
            explanation: "Expenses for months".to_string(),
            expenses_for_months,
        }
    }

    /// Checks if the given month has expenses recorded.
    ///
    /// # Examples
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// let month = YearAndMonth::january(2024);
    /// let expenses = ExpensedMonths::new(IndexMap::from_iter([
    ///     (month.clone(), vec![Item::sample_expense_coffee()]),
    /// ]));
    /// assert!(expenses.contains(&month));
    /// assert!(!expenses.contains(&YearAndMonth::february(2024)));
    /// ```
    pub fn contains(&self, month: &YearAndMonth) -> bool {
        self.expenses_for_months.contains_key(month)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expensed_months_contains() {
        let month = YearAndMonth::january(2024);
        let expenses = ExpensedMonths::new(IndexMap::from_iter([(
            month.clone(),
            vec![Item::sample_expense_coffee()],
        )]));
        assert!(expenses.contains(&month));
        assert!(!expenses.contains(&YearAndMonth::february(2024)));
    }
}
