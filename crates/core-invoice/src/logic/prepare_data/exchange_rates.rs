use std::str::FromStr;

use crate::{Currency, Error, ExchangeRates, FetchExchangeRates, Item, Result, UnitPrice};
use indexmap::IndexMap;

pub type ExchangeRatesMap = IndexMap<Currency, UnitPrice>;
pub type ExchangeRatesFetcher<T = ()> = klirr_foundation::ExchangeRatesFetcher<T>;

impl<T> FetchExchangeRates for ExchangeRatesFetcher<T> {
    fn fetch_for_items(
        &self,
        target_currency: Currency,
        items: Vec<Item>,
    ) -> Result<ExchangeRates> {
        let items = items
            .into_iter()
            .map(|item| {
                klirr_foundation::ExchangeRateItem::builder()
                    .transaction_date(item.transaction_date().to_datetime().date())
                    .source_currency(item.currency().to_string())
                    .build()
            })
            .collect();

        let fetched = self
            .fetch_for_items(target_currency.to_string(), items)
            .map_err(map_exchange_rates_error)?;

        let rates = fetched
            .into_iter()
            .map(|(currency_code, rate)| {
                let currency = Currency::from_str(&currency_code)
                    .map_err(Error::parse_error("Parse currency code"))?;
                Ok((currency, UnitPrice::from(rate)))
            })
            .collect::<Result<ExchangeRatesMap>>()?;

        Ok(ExchangeRates::builder()
            .target_currency(target_currency)
            .rates(rates)
            .build())
    }
}

fn map_exchange_rates_error(error: klirr_foundation::ExchangeRatesError) -> Error {
    match error {
        klirr_foundation::ExchangeRatesError::NetworkError { underlying } => {
            Error::network_error("Fetch exchange rate")(underlying)
        }
        klirr_foundation::ExchangeRatesError::ParseError { underlying } => {
            Error::parse_error("Parse exchange rate response")(underlying)
        }
        klirr_foundation::ExchangeRatesError::MissingRate { target, base } => {
            match (Currency::from_str(&target), Currency::from_str(&base)) {
                (Ok(target), Ok(base)) => Error::FoundNoExchangeRate { target, base },
                _ => Error::parse_error("Parse currency code")(format!(
                    "target='{target}', base='{base}'"
                )),
            }
        }
    }
}
