use crate::prelude::*;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash, Getters, Builder)]
pub struct LineItemsFlat {
    /// True if this invoice is for expenses only.
    #[getset(get = "pub")]
    is_expenses: bool,

    /// Either a single item (Service) or one or more expenses
    #[getset(get = "pub")]
    items: Vec<ItemConvertedIntoTargetCurrency>,
}

impl MaybeIsExpenses for LineItemsFlat {
    fn is_expenses(&self) -> bool {
        self.is_expenses
    }
}

impl TryFrom<(LineItemsPricedInSourceCurrency, ExchangeRates)> for LineItemsFlat {
    type Error = crate::prelude::Error;

    /// Converts the line items priced in source currency into a flat list of items
    /// priced in the target currency, using the provided exchange rates.
    fn try_from(
        (line_items, exchange_rates): (LineItemsPricedInSourceCurrency, ExchangeRates),
    ) -> Result<Self> {
        match line_items {
            LineItemsPricedInSourceCurrency::Service(item) => {
                let service = item.total_cost_in_target_currency(&exchange_rates)?;
                let flat = LineItemsFlat::builder()
                    .items(vec![service])
                    .is_expenses(false)
                    .build();
                Ok(flat)
            }
            LineItemsPricedInSourceCurrency::Expenses(expenses) => {
                let expenses = expenses
                    .into_iter()
                    .map(|expense| expense.total_cost_in_target_currency(&exchange_rates))
                    .collect::<Result<Vec<_>>>()?;
                let flat = LineItemsFlat::builder()
                    .items(expenses)
                    .is_expenses(true)
                    .build();
                Ok(flat)
            }
        }
    }
}

impl HasSample for LineItemsFlat {
    fn sample() -> Self {
        Self::builder()
            .is_expenses(false)
            .items(vec![ItemConvertedIntoTargetCurrency::sample()])
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_line_items_flat_conversion() {
        let line_items = LineItemsPricedInSourceCurrency::sample();
        let exchange_rates = ExchangeRates::builder()
            .rates(ExchangeRatesMap::from_iter([(
                Currency::GBP,
                UnitPrice::from(dec!(10.0)),
            )]))
            .target_currency(Currency::EUR)
            .build();
        let result = LineItemsFlat::try_from((line_items, exchange_rates));
        assert!(
            result.is_ok(),
            "Expected conversion to succeed, got: {:?}",
            result
        );
    }

    #[test]
    fn test_is_expenses() {
        assert!(!MaybeIsExpenses::is_expenses(&LineItemsFlat::sample()));
    }
}
