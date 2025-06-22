use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub struct ExpensedMonths {
    explanation: String,
    #[getset(get = "pub")]
    expenses_for_months: IndexMap<YearAndMonth, Vec<Item>>,
}

impl Default for ExpensedMonths {
    fn default() -> Self {
        Self::new(IndexMap::default())
    }
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

    /// Retrieves the expenses for a specific month from a collection of expenses
    /// organized by `YearAndMonth`.
    ///
    /// /// # Errors
    /// Returns an error if the target month does not have any expenses recorded.
    ///
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// let target_month = YearAndMonth::january(2024);
    /// let expenses_for_months = IndexMap::from_iter([(YearAndMonth::january(2024), vec![Item::sample_expense_breakfast()])]);
    /// let expensed_months = ExpensedMonths::new(expenses_for_months);
    /// let expenses = expensed_months.get(&target_month);
    /// assert_eq!(expenses.unwrap().len(), 1); // January 2024 has one expense
    /// ```
    pub fn get(&self, target_month: &YearAndMonth) -> Result<Vec<Item>> {
        if let Some(items) = self.expenses_for_months().get(target_month) {
            Ok(items.clone())
        } else {
            Err(Error::TargetMonthMustHaveExpenses {
                target_month: *target_month,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expensed_months_contains() {
        let month = YearAndMonth::january(2024);
        let expenses = ExpensedMonths::new(IndexMap::from_iter([(
            month,
            vec![Item::sample_expense_coffee()],
        )]));
        assert!(expenses.contains(&month));
        assert!(!expenses.contains(&YearAndMonth::february(2024)));
    }

    #[test]
    fn test_get_not_found() {
        let expenses = ExpensedMonths::new(IndexMap::new());
        let target_month = YearAndMonth::january(2024);
        let result = expenses.get(&target_month);
        assert!(result.is_err());
    }

    #[test]
    fn test_get() {
        let target_month = YearAndMonth::january(2024);
        let expenses_for_months = IndexMap::from_iter([(
            YearAndMonth::january(2024),
            vec![Item::sample_expense_breakfast()],
        )]);
        let expensed_months = ExpensedMonths::new(expenses_for_months);
        let expenses = expensed_months.get(&target_month);
        assert_eq!(expenses.unwrap().len(), 1); // January 2024 has one expense
    }
}
