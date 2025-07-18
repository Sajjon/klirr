use std::ops::Mul;

use crate::prelude::*;

/// Represents exchange rates for a specific target currency in relation to other currencies.
#[derive(Clone, Debug, Serialize, Builder, Getters)]
pub struct ExchangeRates {
    /// MUST Match the currency of the invoice, e.g. `"EUR"`.
    #[getset(get = "pub")]
    target_currency: Currency,

    /// Exchange rates for the `target_currency` in relation to other currencies.
    /// The keys are the base currencies, and the values are the exchange rates.
    ///
    /// For example, if the `target_currency` is `"EUR"` and the rates are:
    /// ```text
    /// "USD": 1.2,
    /// "GBP": 0.85,
    /// "SEK": 11.0
    /// ```
    /// then 1 USD is 1.2 EUR, 1 GBP is 0.85 EUR, and 1 SEK is 11.0 EUR.
    ///
    #[getset(get = "pub")]
    rates: ExchangeRatesMap,
}

impl ExchangeRates {
    /// Converts a given `unit_price` from the `currency` to the `target_currency`.
    /// If the `currency` is the same as the `target_currency`, it returns the `unit_price
    /// as is.
    /// If the `currency` is not found in the exchange rates, it returns an error.
    /// If the conversion is successful, it returns the converted `UnitPrice`.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let exchange_rates = ExchangeRates::builder()
    ///     .target_currency(Currency::EUR)
    ///     .rates(ExchangeRatesMap::from([
    ///         (Currency::USD, UnitPrice::from(dec!(0.85))),
    ///         (Currency::GBP, UnitPrice::from(dec!(1.1))),
    ///     ]))
    ///     .build();
    /// let converted = exchange_rates.convert(dec!(100.0), Currency::USD).unwrap();
    /// assert_eq!(*converted, dec!(85.0));
    /// ```
    ///
    /// # Errors
    /// Returns an error if the `currency` is not found in the exchange rates.
    ///
    pub fn convert(
        &self,
        unit_price: impl Into<UnitPrice>,
        currency: Currency,
    ) -> Result<UnitPrice> {
        let unit_price = unit_price.into();
        if self.target_currency == currency {
            return Ok(unit_price);
        }
        let rate = self.get_rate(currency)?;
        let converted = rate.mul(*unit_price);
        Ok(converted)
    }

    fn get_rate(&self, currency: Currency) -> Result<UnitPrice> {
        self.rates
            .get(&currency)
            .cloned()
            .ok_or(Error::FoundNoExchangeRate {
                target: self.target_currency,
                base: currency,
            })
    }
}

impl ExchangeRates {
    pub fn hard_coded() -> Self {
        let rates = ExchangeRatesMap::from([
            (Currency::EUR, UnitPrice::from(dec!(1.0))),
            (Currency::USD, UnitPrice::from(dec!(1.2))),
            (Currency::GBP, UnitPrice::from(dec!(0.85))),
            (Currency::SEK, UnitPrice::from(dec!(11.0))),
        ]);
        Self {
            target_currency: Currency::EUR,
            rates,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_hard_coded() {
        let exchange_rates = ExchangeRates::hard_coded();
        assert!(!exchange_rates.rates().is_empty());
    }

    #[test]
    fn test_convert() {
        let exchange_rates = ExchangeRates::hard_coded();
        let converted = exchange_rates.convert(dec!(100.0), Currency::USD).unwrap();
        assert_eq!(*converted, dec!(120.0));
    }

    #[test]
    fn test_convert_not_found() {
        let exchange_rates = ExchangeRates::builder()
            .target_currency(Currency::EUR)
            .rates(ExchangeRatesMap::new())
            .build();
        let result = exchange_rates.convert(dec!(100.0), Currency::JPY);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_rate_not_found() {
        let exchange_rates = ExchangeRates::builder()
            .target_currency(Currency::EUR)
            .rates(ExchangeRatesMap::new())
            .build();
        let result = exchange_rates.get_rate(Currency::JPY);
        assert!(result.is_err());
    }
}
