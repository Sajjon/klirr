use crate::{define_item_struct, prelude::*};

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub struct ExpensedMonths {
    explanation: String,
    #[getset(get = "pub")]
    expenses_for_months: IndexMap<YearAndMonth, ExpensesForMonth>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(transparent)]
struct ExpensesForMonth(Vec<Item>);

#[derive(Hash, Eq, PartialEq, Display, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
struct QuantityIgnored;
define_item_struct!(pub, Marker, QuantityIgnored);

impl ExpensesForMonth {
    /// Inserts a vector of items into the `ExpensesForMonth`, merging items that are the same
    /// except for their quantity.
    fn insert(&mut self, items: Vec<Item>) {
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
                .name(marker.name())
                .transaction_date(*marker.transaction_date())
                .unit_price(*marker.unit_price())
                .currency(*marker.currency())
                .quantity(quantity)
                .build();
            self.0.push(item);
        }
    }

    /// Creates a new `ExpensesForMonth` with the given items, merging items that are the same
    /// except for their quantity.
    fn new(items: Vec<Item>) -> Self {
        let mut self_ = Self::default();
        self_.insert(items);
        self_
    }

    /// Returns the items in this month
    fn items(&self) -> Vec<Item> {
        self.0.clone()
    }
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
            expenses_for_months: expenses_for_months
                .into_iter()
                .map(|(month, items)| (month, ExpensesForMonth::new(items)))
                .collect(),
        }
    }

    /// Checks if the given month has expenses recorded.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
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
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let target_month = YearAndMonth::january(2024);
    /// let expenses_for_months = IndexMap::from_iter([(YearAndMonth::january(2024), vec![Item::sample_expense_breakfast()])]);
    /// let expensed_months = ExpensedMonths::new(expenses_for_months);
    /// let expenses = expensed_months.get(&target_month);
    /// assert_eq!(expenses.unwrap().len(), 1); // January 2024 has one expense
    /// ```
    pub fn get(&self, target_month: &YearAndMonth) -> Result<Vec<Item>> {
        if let Some(items) = self.expenses_for_months().get(target_month) {
            Ok(items.items())
        } else {
            Err(Error::TargetMonthMustHaveExpenses {
                target_month: *target_month,
            })
        }
    }

    /// Inserts expenses for a specific month into the collection of expenses.
    /// If the month already exists, it merges the new items with the existing ones,
    /// ensuring that items that are the same except for their quantity are combined.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let mut expensed_months = ExpensedMonths::new(IndexMap::new());
    /// let month = YearAndMonth::january(2024);
    /// let items = vec![Item::sample_expense_coffee()];
    /// expensed_months.insert_expenses(&month, items.clone());
    /// assert_eq!(expensed_months.get(&month).unwrap().len(), 1);
    /// assert_eq!(*expensed_months.get(&month).unwrap()[0].quantity(), Quantity::from(2.0));
    /// expensed_months.insert_expenses(&month, items.clone());
    /// assert_eq!(expensed_months.get(&month).unwrap().len(), 1); // Still one item, but quantity is now 4.0
    /// assert_eq!(*expensed_months.get(&month).unwrap()[0].quantity(), Quantity::from(4.0)); // 2 + 2
    ///
    /// ```
    pub fn insert_expenses(&mut self, month: &YearAndMonth, items: Vec<Item>) {
        match self.expenses_for_months.entry(*month) {
            indexmap::map::Entry::Occupied(mut entry) => {
                entry.get_mut().insert(items);
            }
            indexmap::map::Entry::Vacant(entry) => {
                entry.insert(ExpensesForMonth::new(items));
            }
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

    #[test]
    fn test_insert_expenses_different() {
        let mut expensed_months = ExpensedMonths::new(IndexMap::new());
        let month = YearAndMonth::january(2024);
        let items = vec![Item::sample_expense_coffee()];
        expensed_months.insert_expenses(&month, items.clone());
        assert!(expensed_months.contains(&month));
        let retrieved_items = expensed_months.get(&month).unwrap();
        assert_eq!(retrieved_items.len(), 1);
    }

    #[test]
    fn test_insert_expenses_same_except_quantity() {
        let mut expensed_months = ExpensedMonths::new(IndexMap::new());
        let month = YearAndMonth::january(2024);
        let item1 = Item::builder()
            .name("Coffee")
            .unit_price(UnitPrice::from(2.5))
            .currency(Currency::EUR)
            .quantity(Quantity::from(3.0))
            .transaction_date(Date::from_str("2024-01-01").unwrap())
            .build();
        let item2 = Item::builder()
            .name("Coffee")
            .unit_price(UnitPrice::from(2.5))
            .currency(Currency::EUR)
            .quantity(Quantity::from(4.0))
            .transaction_date(Date::from_str("2024-01-01").unwrap())
            .build();
        expensed_months.insert_expenses(&month, vec![item1.clone(), item2.clone()]);
        assert!(expensed_months.contains(&month));
        let retrieved_items = expensed_months.get(&month).unwrap();
        assert_eq!(retrieved_items.len(), 1);
        assert_eq!(*retrieved_items[0].quantity(), Quantity::from(7.0)); // 3.0 + 4.0
    }

    #[test]
    fn test_insert_expenses_same_except_quantity_added_in_two_batches() {
        let mut expensed_months = ExpensedMonths::new(IndexMap::new());
        let month = YearAndMonth::january(2024);
        let item1 = Item::builder()
            .name("Coffee")
            .unit_price(UnitPrice::from(2.5))
            .currency(Currency::EUR)
            .quantity(Quantity::from(3.0))
            .transaction_date(Date::from_str("2024-01-01").unwrap())
            .build();
        let item2 = Item::builder()
            .name("Coffee")
            .unit_price(UnitPrice::from(2.5))
            .currency(Currency::EUR)
            .quantity(Quantity::from(4.0))
            .transaction_date(Date::from_str("2024-01-01").unwrap())
            .build();
        expensed_months.insert_expenses(&month, vec![item1.clone()]);
        expensed_months.insert_expenses(&month, vec![item2.clone()]);
        assert!(expensed_months.contains(&month));
        let retrieved_items = expensed_months.get(&month).unwrap();
        assert_eq!(retrieved_items.len(), 1);
        assert_eq!(*retrieved_items[0].quantity(), Quantity::from(7.0)); // 3.0 + 4.0
    }
}
