use super::expenses_for_periods::ExpensesForPeriods;
use crate::{Date, Error, HasSample, Item, Result};
use getset::Getters;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Period-end dates for which expenses have been recorded.
#[derive(Clone, Debug, Serialize, PartialEq, Deserialize, Getters)]
pub struct ExpensedPeriods {
    explanation: String,
    #[getset(get = "pub")]
    expenses_for_periods: IndexMap<Date, ExpensesForPeriods>,
}

impl HasSample for ExpensedPeriods {
    fn sample() -> Self {
        Self::new(IndexMap::from_iter([(
            Date::sample(),
            vec![Item::sample_expense_breakfast()],
        )]))
    }

    fn sample_other() -> Self {
        Self::new(IndexMap::from_iter([(
            Date::sample_other(),
            vec![Item::sample_expense_coffee()],
        )]))
    }
}

impl Default for ExpensedPeriods {
    fn default() -> Self {
        Self::new(IndexMap::default())
    }
}

impl ExpensedPeriods {
    pub fn new(expenses_for_periods: IndexMap<Date, Vec<Item>>) -> Self {
        Self {
            explanation: "Expenses for periods".to_string(),
            expenses_for_periods: expenses_for_periods
                .into_iter()
                .map(|(period_end_date, items)| (period_end_date, ExpensesForPeriods::new(items)))
                .collect(),
        }
    }

    pub fn contains(&self, period_end_date: &Date) -> bool {
        self.expenses_for_periods.contains_key(period_end_date)
    }

    pub fn get(&self, target_period_end_date: &Date) -> Result<Vec<Item>> {
        if let Some(items) = self.expenses_for_periods().get(target_period_end_date) {
            Ok(items.items())
        } else {
            Err(Error::TargetPeriodMustHaveExpenses {
                target_period: target_period_end_date.to_string(),
            })
        }
    }

    pub fn insert_expenses(&mut self, period_end_date: &Date, items: Vec<Item>) {
        match self.expenses_for_periods.entry(*period_end_date) {
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
    use crate::{Currency, Quantity, UnitPrice};
    use rust_decimal::dec;
    use std::str::FromStr;

    type Sut = ExpensedPeriods;

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
    fn test_get_not_found() {
        let expenses = ExpensedPeriods::sample();
        let target_period_end_date = Date::from_str("1970-01-31").unwrap();
        let result = expenses.get(&target_period_end_date);
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_expenses_same_except_quantity_added_in_two_batches() {
        let mut expensed_periods = ExpensedPeriods::new(IndexMap::new());
        let period_end_date = Date::from_str("2024-01-31").unwrap();
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
        expensed_periods.insert_expenses(&period_end_date, vec![item1.clone()]);
        expensed_periods.insert_expenses(&period_end_date, vec![item2.clone()]);
        assert!(expensed_periods.contains(&period_end_date));
        let retrieved_items = expensed_periods.get(&period_end_date).unwrap();
        assert_eq!(retrieved_items.len(), 1);
        assert_eq!(*retrieved_items[0].quantity(), Quantity::from(dec!(7.0))); // 3.0 + 4.0
    }
}
