use std::str::FromStr;

use crate::prelude::*;

#[macro_export]
macro_rules! define_item_struct {
    ($vis:vis, $name:ident, $quantity_ty:ty) => {
        #[derive(
            Clone,
            Debug,
            Display,
            PartialEq,
            Eq,
            Hash,
            Serialize,
            Deserialize,
            Getters,
            Setters,
            Builder,
        )]
        #[display(
            "{}: {}{} #{} @{}",
            name,
            unit_price,
            currency,
            quantity,
            transaction_date
        )]
        $vis struct $name {
            /// The short name of the expense, e.g. `"Coffee"`
            #[getset(get = "pub")]
            name: String,
            /// The cost per item
            #[getset(get = "pub")]
            unit_price: UnitPrice,
            /// The currency of the expense, e.g. `"EUR"`, this
            /// is the currency in which the expense was paid,
            /// and not necessarily the currency of the invoice.
            #[getset(get = "pub")]
            currency: Currency,
            /// The quantity of the expense, e.g. `2.0` for two items
            #[getset(get = "pub", set)]
            quantity: $quantity_ty,
            /// The date of the expense, e.g. `2025-05-31`
            #[getset(get = "pub")]
            transaction_date: Date,
        }
    };
}

define_item_struct!(pub, Item, Quantity);

impl HasSample for Item {
    fn sample() -> Self {
        Self::sample_expense_coffee()
    }
    fn sample_other() -> Self {
        Self::sample_consulting_service()
    }
}

impl Item {
    pub fn sample_expense_breakfast() -> Self {
        Self::builder()
            .name("Breakfast".into())
            .transaction_date(
                Date::builder()
                    .year(2025.into())
                    .month(Month::May)
                    .day(Day::try_from(20).unwrap())
                    .build(),
            )
            .quantity(dec!(1.0).into())
            .unit_price(dec!(145.0).into())
            .currency(Currency::SEK)
            .build()
    }

    pub fn sample_expense_coffee() -> Self {
        Self::builder()
            .name("Coffee".into())
            .transaction_date(Date::sample())
            .quantity(dec!(2.0).into())
            .unit_price(dec!(4.0).into())
            .currency(Currency::GBP)
            .build()
    }

    pub fn sample_consulting_service() -> Self {
        Self::builder()
            .name("Agreed Consulting Fees".into())
            .transaction_date(Date::sample())
            .quantity(dec!(22.0).into())
            .unit_price(dec!(500.0).into())
            .currency(Currency::EUR)
            .build()
    }
}

impl Item {
    /// Converts the item into a new item with the total cost calculated
    /// in the target currency, using the provided exchange rates.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let item = Item::from_str("Coffee,2.5, EUR,3.0, 2025-05-31").expect("Failed to parse Item");
    /// let exchange_rates = ExchangeRates::builder()
    ///     .target_currency(Currency::USD)
    ///     .rates(ExchangeRatesMap::from([
    ///         (Currency::EUR, UnitPrice::from(dec!(1.2))),
    ///         (Currency::GBP, UnitPrice::from(dec!(1.5))),
    ///     ]))
    ///     .build();
    /// let converted_item = item.total_cost_in_target_currency(&exchange_rates).expect("Failed to convert item");
    /// assert_eq!(converted_item.name(), "Coffee");
    /// assert_eq!(**converted_item.unit_price(), dec!(3.0)); // EUR to USD conversion
    /// assert_eq!(converted_item.currency(), &Currency::USD);
    /// ```
    pub fn total_cost_in_target_currency(
        self,
        exchange_rates: &ExchangeRates,
    ) -> Result<ItemConvertedIntoTargetCurrency> {
        let converted_rates = self.with_exchange_rates(exchange_rates)?;
        Ok(converted_rates.with_total_cost())
    }

    /// Maps an `Item` into an `ItemConvertedIntoTargetCurrency` with the total cost
    /// calculated in the source currency.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let item = Item::from_str("Coffee,2.5, EUR,3.0, 2025-05-31").expect("Failed to parse Item");
    /// let converted_item = item.with_total_cost();
    /// assert_eq!(**converted_item.total_cost(), dec!(7.50)); // 2.5 * 3.0
    /// ```
    pub fn with_total_cost(self) -> ItemConvertedIntoTargetCurrency {
        let cost = Cost::from(**self.quantity() * **self.unit_price());
        ItemConvertedIntoTargetCurrency::builder()
            .in_source_currency(self.clone())
            .total_cost(cost)
            .build()
    }

    /// Converts the item into a new item with the unit price converted to the target currency
    /// using the provided exchange rates.
    fn with_exchange_rates(self, exchange_rates: &ExchangeRates) -> Result<Self> {
        let converted_unit_price = exchange_rates.convert(self.unit_price, self.currency)?;
        Ok(Self::builder()
            .transaction_date(self.transaction_date)
            .name(self.name)
            .unit_price(converted_unit_price)
            .quantity(self.quantity)
            .currency(*exchange_rates.target_currency())
            .build())
    }
}

impl FromStr for Item {
    type Err = crate::prelude::Error;

    /// Parses a string in the format: "name, unit_price, currency, quantity, transaction_date", or
    /// without spaces after commas, even mixed, e.g. "Coffee, 2.5,EUR, 3.0,2025-05-31".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();
        if parts.len() != 5 {
            return Err(Error::InvalidExpenseItem {
                invalid_string: s.to_string(),
                reason: "Expected 5 comma-separated values, on format: \"Coffee, 2.5, EUR, 3.0, 2025-05-31\"".to_string(),
            });
        }

        let name = parts[0].to_string();
        let unit_price: UnitPrice = parts[1]
            .parse::<Decimal>()
            .map_err(|e| Error::InvalidExpenseItem {
                invalid_string: s.to_string(),
                reason: format!("Failed to parse unit_price: {e}"),
            })?
            .into();

        let currency = Currency::from_str(parts[2]).map_err(|e| Error::InvalidExpenseItem {
            invalid_string: s.to_string(),
            reason: format!("Failed to parse currency: {e}"),
        })?;

        let quantity: Quantity = parts[3]
            .parse::<Decimal>()
            .map_err(|e| Error::InvalidExpenseItem {
                invalid_string: s.to_string(),
                reason: format!("Failed to parse quantity: {e}"),
            })?
            .into();
        if quantity < Quantity::ZERO {
            return Err(Error::InvalidExpenseItem {
                invalid_string: s.to_string(),
                reason: "Quantity cannot be negative".to_string(),
            });
        }

        let transaction_date = Date::from_str(parts[4]).map_err(|e| Error::InvalidExpenseItem {
            invalid_string: s.to_string(),
            reason: format!("Failed to parse transaction_date: {e}"),
        })?;

        Ok(Item::builder()
            .name(name)
            .unit_price(unit_price)
            .currency(currency)
            .quantity(quantity)
            .transaction_date(transaction_date)
            .build())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_log::test;

    type Sut = Item;

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
    fn test_from_str() {
        // N.B. sometimes space after comma, sometimes not.
        let sut = Sut::from_str("Coffee,2.5, EUR,3.0, 2025-05-31").expect("Failed to parse Item");
        assert_eq!(sut.name(), "Coffee");
        assert_eq!(**sut.unit_price(), dec!(2.5));
        assert_eq!(sut.currency(), &Currency::EUR);
        assert_eq!(**sut.quantity(), dec!(3.0));
        assert_eq!(
            sut.transaction_date(),
            &Date::from_str("2025-05-31").unwrap()
        );
    }

    #[test]
    fn from_str_invalid() {
        let invalid_strings = [
            "Coffee,2.5, EUR,3.0",                          // Missing transaction_date
            "Coffee,2.5, EUR,3.0, invalid_date",            // Invalid transaction_date
            "Coffee,2.5, EUR,3.0, 2025-05-31, extra",       // Too many parts
            "Coffee,invalid_price, EUR,3.0, 2025-05-31",    // Invalid unit_price
            "Coffee,2.5, invalid_currency,3.0, 2025-05-31", // Invalid currency
            "Coffee,2.5, EUR,-3.0, 2025-05-31",             // Negative quantity
            "Coffee,2.5, EUR,a, 2025-05-31",                // Negative quantity
        ];

        for &s in &invalid_strings {
            assert!(Sut::from_str(s).is_err(), "Expected error for: {}", s);
        }
    }
}
