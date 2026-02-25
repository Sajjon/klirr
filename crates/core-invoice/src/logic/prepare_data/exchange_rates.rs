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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;
    use tempfile::tempdir;

    #[test]
    fn fetch_for_items_maps_foundation_rates_to_core_types() {
        let tempdir = tempdir().unwrap();
        let item = Item::sample_expense_coffee();

        let fetcher = ExchangeRatesFetcher::builder()
            .path_to_cache(tempdir.path().to_path_buf())
            .extra(())
            .build();

        // Same source and target currency makes the foundation fetcher deterministic
        // and avoids hitting the network.
        let rates =
            FetchExchangeRates::fetch_for_items(&fetcher, Currency::GBP, vec![item]).unwrap();

        assert_eq!(rates.target_currency(), &Currency::GBP);
        assert_eq!(rates.rates().len(), 1);
        assert_eq!(
            rates.rates().get(&Currency::GBP),
            Some(&UnitPrice::from(dec!(1.0)))
        );
    }

    #[test]
    fn map_exchange_rates_error_maps_network_error() {
        let error = map_exchange_rates_error(klirr_foundation::ExchangeRatesError::NetworkError {
            underlying: "timed out".to_string(),
        });
        assert_eq!(
            error,
            Error::NetworkError {
                underlying: "Fetch exchange rate: timed out".to_string(),
            }
        );
    }

    #[test]
    fn map_exchange_rates_error_maps_parse_error() {
        let error = map_exchange_rates_error(klirr_foundation::ExchangeRatesError::ParseError {
            underlying: "unexpected eof".to_string(),
        });
        assert_eq!(
            error,
            Error::ParseError {
                underlying: "Parse exchange rate response: unexpected eof".to_string(),
            }
        );
    }

    #[test]
    fn map_exchange_rates_error_maps_missing_rate_when_currency_codes_are_valid() {
        let error = map_exchange_rates_error(klirr_foundation::ExchangeRatesError::MissingRate {
            target: "EUR".to_string(),
            base: "GBP".to_string(),
        });
        assert_eq!(
            error,
            Error::FoundNoExchangeRate {
                target: Currency::EUR,
                base: Currency::GBP,
            }
        );
    }

    #[test]
    fn map_exchange_rates_error_maps_missing_rate_to_parse_error_when_codes_are_invalid() {
        let error = map_exchange_rates_error(klirr_foundation::ExchangeRatesError::MissingRate {
            target: "NOT_A_CURRENCY".to_string(),
            base: "ALSO_BAD".to_string(),
        });
        assert_eq!(
            error,
            Error::ParseError {
                underlying: "Parse currency code: target='NOT_A_CURRENCY', base='ALSO_BAD'"
                    .to_string(),
            }
        );
    }
}
