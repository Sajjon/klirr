use crate::prelude::*;

#[derive(Clone, Debug, Serialize, TypedBuilder, Getters)]
pub struct ExchangeRates {
    /// MUST Match the currency of the invoice, e.g. `"EUR"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    target_currency: Currency,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    rates: HashMap<Currency, UnitPrice>,
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
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// let exchange_rates = ExchangeRates::builder()
    ///     .target_currency(Currency::EUR)
    ///     .rates(HashMap::from([
    ///         (Currency::USD, UnitPrice::from(0.85)),
    ///         (Currency::GBP, UnitPrice::from(1.1)),
    ///     ]))
    ///     .build();
    /// let converted = exchange_rates.convert(100.0, Currency::USD).unwrap();
    /// assert_eq!(*converted, 85.0);
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
        let rate = self
            .rates
            .get(&currency)
            .ok_or(Error::FoundNoExchangeRate {
                target: self.target_currency,
                base: currency,
            })?;
        Ok(UnitPrice::from(*unit_price * **rate))
    }
}

impl ExchangeRates {
    pub fn hard_coded() -> Self {
        let rates = HashMap::from([
            (Currency::EUR, UnitPrice::from(1.0)),
            (Currency::USD, UnitPrice::from(1.2)),
            (Currency::GBP, UnitPrice::from(0.85)),
            (Currency::SEK, UnitPrice::from(11.0)),
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
        assert_eq!(exchange_rates.rates().is_empty(), false);
    }
}
