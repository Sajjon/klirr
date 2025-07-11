use crate::{define_item_struct, prelude::*};

#[derive(Clone, Debug, Serialize, PartialEq, Deserialize, Getters)]
pub struct ExpensedPeriods<Period: IsPeriod> {
    explanation: String,
    #[getset(get = "pub")]
    expenses_for_periods: IndexMap<Period, ExpensesForPeriods>,
}

impl<Period: IsPeriod + HasSample> HasSample for ExpensedPeriods<Period> {
    fn sample() -> Self {
        Self::new(IndexMap::from_iter([(
            Period::sample(),
            vec![Item::sample_expense_breakfast()],
        )]))
    }
    fn sample_other() -> Self {
        Self::new(IndexMap::from_iter([(
            Period::sample_other(),
            vec![Item::sample_expense_coffee()],
        )]))
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Deserialize, Default)]
#[serde(transparent)]
struct ExpensesForPeriods(Vec<Item>);

#[derive(Hash, Eq, PartialEq, Display, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
struct QuantityIgnored;
define_item_struct!(pub, Marker, QuantityIgnored);

impl ExpensesForPeriods {
    /// Inserts a vector of items into the `ExpensesForPeriods`, merging items that are the same
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

impl<Period: IsPeriod> Default for ExpensedPeriods<Period> {
    fn default() -> Self {
        Self::new(IndexMap::default())
    }
}

impl<Period: IsPeriod> ExpensedPeriods<Period> {
    pub fn new(expenses_for_periods: IndexMap<Period, Vec<Item>>) -> Self {
        Self {
            explanation: "Expenses for periods".to_string(),
            expenses_for_periods: expenses_for_periods
                .into_iter()
                .map(|(month, items)| (month, ExpensesForPeriods::new(items)))
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
    /// let expenses = ExpensedPeriods::new(IndexMap::from_iter([
    ///     (month.clone(), vec![Item::sample_expense_coffee()]),
    /// ]));
    /// assert!(expenses.contains(&month));
    /// assert!(!expenses.contains(&YearAndMonth::february(2024)));
    /// ```
    pub fn contains(&self, period: &Period) -> bool {
        self.expenses_for_periods.contains_key(period)
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
    /// let expensed_months = ExpensedPeriods::new(expenses_for_months);
    /// let expenses = expensed_months.get(&target_month);
    /// assert_eq!(expenses.unwrap().len(), 1); // January 2024 has one expense
    /// ```
    pub fn get(&self, target_period: &Period) -> Result<Vec<Item>> {
        if let Some(items) = self.expenses_for_periods().get(target_period) {
            Ok(items.items())
        } else {
            Err(Error::TargetPeriodMustHaveExpenses {
                target_period: format!("{:?}", target_period),
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
    /// let mut expensed_months = ExpensedPeriods::new(IndexMap::new());
    /// let month = YearAndMonth::january(2024);
    /// let items = vec![Item::sample_expense_coffee()];
    /// expensed_months.insert_expenses(&month, items.clone());
    /// assert_eq!(expensed_months.get(&month).unwrap().len(), 1);
    /// assert_eq!(*expensed_months.get(&month).unwrap()[0].quantity(), Quantity::from(dec!(2.0)));
    /// expensed_months.insert_expenses(&month, items.clone());
    /// assert_eq!(expensed_months.get(&month).unwrap().len(), 1); // Still one item, but quantity is now 4.0
    /// assert_eq!(*expensed_months.get(&month).unwrap()[0].quantity(), Quantity::from(dec!(4.0))); // 2 + 2
    ///
    /// ```
    pub fn insert_expenses(&mut self, period: &Period, items: Vec<Item>) {
        match self.expenses_for_periods.entry(period.clone()) {
            indexmap::map::Entry::Occupied(mut entry) => {
                entry.get_mut().insert(items);
            }
            indexmap::map::Entry::Vacant(entry) => {
                entry.insert(ExpensesForPeriods::new(items));
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
        let expenses = ExpensedPeriods::new(IndexMap::from_iter([(
            month,
            vec![Item::sample_expense_coffee()],
        )]));
        assert!(expenses.contains(&month));
        assert!(!expenses.contains(&YearAndMonth::february(2024)));
    }

    #[test]
    fn test_get_not_found() {
        let expenses = ExpensedPeriods::sample();
        let target_month = YearAndMonth::january(1970);
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
        let expensed_months = ExpensedPeriods::new(expenses_for_months);
        let expenses = expensed_months.get(&target_month);
        assert_eq!(expenses.unwrap().len(), 1); // January 2024 has one expense
    }

    #[test]
    fn test_insert_expenses_different() {
        let mut expensed_months = ExpensedPeriods::new(IndexMap::new());
        let month = YearAndMonth::january(2024);
        let items = vec![Item::sample_expense_coffee()];
        expensed_months.insert_expenses(&month, items.clone());
        assert!(expensed_months.contains(&month));
        let retrieved_items = expensed_months.get(&month).unwrap();
        assert_eq!(retrieved_items.len(), 1);
    }

    #[test]
    fn test_insert_expenses_same_except_quantity() {
        let mut expensed_months = ExpensedPeriods::new(IndexMap::new());
        let month = YearAndMonth::january(2024);
        let item1 = Item::builder()
            .name("Coffee".into())
            .unit_price(UnitPrice::from(dec!(2.5)))
            .currency(Currency::EUR)
            .quantity(Quantity::from(dec!(3.0)))
            .transaction_date(Date::from_str("2024-01-01").unwrap())
            .build();
        let item2 = Item::builder()
            .name("Coffee".into())
            .unit_price(UnitPrice::from(dec!(2.5)))
            .currency(Currency::EUR)
            .quantity(Quantity::from(dec!(4.0)))
            .transaction_date(Date::from_str("2024-01-01").unwrap())
            .build();
        expensed_months.insert_expenses(&month, vec![item1.clone(), item2.clone()]);
        assert!(expensed_months.contains(&month));
        let retrieved_items = expensed_months.get(&month).unwrap();
        assert_eq!(retrieved_items.len(), 1);
        assert_eq!(*retrieved_items[0].quantity(), Quantity::from(dec!(7.0))); // 3.0 + 4.0
    }

    #[test]
    fn test_insert_expenses_same_except_quantity_added_in_two_batches() {
        let mut expensed_months = ExpensedPeriods::new(IndexMap::new());
        let month = YearAndMonth::january(2024);
        let item1 = Item::builder()
            .name("Coffee".into())
            .unit_price(UnitPrice::from(dec!(2.5)))
            .currency(Currency::EUR)
            .quantity(Quantity::from(dec!(3.0)))
            .transaction_date(Date::from_str("2024-01-01").unwrap())
            .build();
        let item2 = Item::builder()
            .name("Coffee".into())
            .unit_price(UnitPrice::from(dec!(2.5)))
            .currency(Currency::EUR)
            .quantity(Quantity::from(dec!(4.0)))
            .transaction_date(Date::from_str("2024-01-01").unwrap())
            .build();
        expensed_months.insert_expenses(&month, vec![item1.clone()]);
        expensed_months.insert_expenses(&month, vec![item2.clone()]);
        assert!(expensed_months.contains(&month));
        let retrieved_items = expensed_months.get(&month).unwrap();
        assert_eq!(retrieved_items.len(), 1);
        assert_eq!(*retrieved_items[0].quantity(), Quantity::from(dec!(7.0))); // 3.0 + 4.0
    }

    #[test]
    fn default_is_empty() {
        let expensed_periods = ExpensedPeriods::<YearAndMonth>::default();
        assert!(expensed_periods.expenses_for_periods.is_empty());
    }
}
