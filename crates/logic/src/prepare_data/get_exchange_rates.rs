use std::collections::HashMap;

use crate::prelude::*;

/// Response has format:
/// ```json
/// {
///   "amount": 1.0,
///   "base": "GBP",
///   "date": "2025-04-30",
///   "rates": {
///     "EUR": 1.174
///   }
///  }
/// ```
/// as given by `curl -s "https://api.frankfurter.app/2025-05-01?from=GBP&to=EUR"`
#[derive(Debug, Clone, Deserialize, Getters)]
struct FrankfurterApiResponse {
    #[getset(get = "pub")]
    rates: HashMap<Currency, f64>,
}

trait DeserializableResponse {
    fn json<T: serde::de::DeserializeOwned>(self) -> Result<T>;
}
impl DeserializableResponse for reqwest::blocking::Response {
    fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        self.json().map_err(|e| {
            let msg = format!("Failed to parse response JSON: {}", e);
            error!("{}", msg);
            Error::ParseError { underlying: msg }
        })
    }
}

/// Makes blocking requests to the Frankfurter API to get the exchange rate
fn get_exchange_rate(date: Date, from: Currency, to: Currency) -> Result<UnitPrice> {
    get_exchange_rate_with_fetcher(date, from, to, |url| {
        reqwest::blocking::get(&url).map_err(|e| {
            let msg = format!("Failed to fetch exchange rate from {}: {}", url, e);
            error!("{}", msg);
            Error::NetworkError { underlying: msg }
        })
    })
}

/// Makes blocking requests to the Frankfurter API to get the exchange rate
fn get_exchange_rate_with_fetcher<T: DeserializableResponse>(
    date: Date,
    from: Currency,
    to: Currency,
    fetcher: impl Fn(String) -> Result<T>,
) -> Result<UnitPrice> {
    if from == to {
        return Ok(UnitPrice::from(1.0));
    }
    debug!(
        "Fetching exchange rate for {} from {} to {}",
        date, from, to
    );
    let url = format!(
        "https://api.frankfurter.app/{}?from={}&to={}",
        date, from, to
    );
    fetcher(url)?
        .json::<FrankfurterApiResponse>()
        .and_then(|response| {
            response
                .rates()
                .get(&to)
                .cloned()
                .ok_or(Error::FoundNoExchangeRate {
                    target: to,
                    base: from,
                })
                .map(UnitPrice::from)
        })
}

pub type ExchangeRatesMap = HashMap<Currency, UnitPrice>;

pub fn get_exchange_rates_if_needed(
    target_currency: Currency,
    items: &LineItemsPricedInSourceCurrency,
) -> Result<ExchangeRates> {
    get_exchange_rates_if_needed_with_fetcher(target_currency, items, get_exchange_rate)
}

/// Fetches exchange rates for expenses in the provided line items if needed.
fn get_exchange_rates_if_needed_with_fetcher(
    target_currency: Currency,
    items: &LineItemsPricedInSourceCurrency,
    get_exchange_rate: impl Fn(Date, Currency, Currency) -> Result<UnitPrice>,
) -> Result<ExchangeRates> {
    let Ok(expenses) = items.clone().try_unwrap_expenses() else {
        debug!("No expenses found, skipping exchange rate fetching.");
        return Ok(ExchangeRates::builder()
            .target_currency(target_currency)
            .rates(HashMap::new())
            .build());
    };
    debug!(
        "☑️ Fetching exchanges rates for #{} expenses...",
        expenses.len()
    );
    let mut rates = HashMap::new();
    for expense in expenses {
        let from = *expense.currency();
        if let std::collections::hash_map::Entry::Vacant(e) = rates.entry(from) {
            let date = *expense.transaction_date();
            let rate = get_exchange_rate(date, from, target_currency)?;
            e.insert(rate);
        }
    }
    debug!("✅ Fetched exchanges rates for #{} expenses.", rates.len());
    let rates = ExchangeRates::builder()
        .target_currency(target_currency)
        .rates(rates)
        .build();
    Ok(rates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_frankfurter_api_response() {
        let response = r#"{
            "amount": 1.0,
            "base": "GBP",
            "date": "2025-04-30",
            "rates": {
                "EUR": 1.174
            }
        }"#;

        let parsed: FrankfurterApiResponse = serde_json::from_str(response).unwrap();
        assert_eq!(
            parsed.rates.get(&Currency::EUR).unwrap().to_string(),
            "1.174"
        );
    }

    struct Mock<'a> {
        json: &'a str,
    }
    impl DeserializableResponse for Mock<'_> {
        fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
            serde_json::from_str(self.json).map_err(|e| {
                let msg = format!("Failed to parse mock response JSON: {}", e);
                error!("{}", msg);
                Error::ParseError { underlying: msg }
            })
        }
    }

    #[test]
    fn test_get_exchange_rate() {
        let date = Date::from_str("2025-04-30").unwrap();
        let from = Currency::GBP;
        let to = Currency::EUR;
        let rate = get_exchange_rate_with_fetcher(date, from, to, |url| {
            assert_eq!(
                url,
                "https://api.frankfurter.app/2025-04-30?from=GBP&to=EUR"
            );
            // Mocking the fetcher to return a predefined response
            let response = r#"{
                "amount": 1.0,
                "base": "GBP",
                "date": "2025-04-30",
                "rates": {
                    "EUR": 1.174
                }
            }"#;
            Ok(Mock { json: response })
        });
        assert!(rate.is_ok());
    }

    #[test]
    fn test_get_exchange_rates_if_needed() {
        let items = LineItemsPricedInSourceCurrency::Expenses(vec![
            Item::builder()
                .name("Coffee")
                .transaction_date(Date::sample())
                .quantity(2.0)
                .unit_price(4.0)
                .currency(Currency::GBP)
                .build(),
        ]);
        let target_currency = Currency::EUR;
        let rates =
            get_exchange_rates_if_needed_with_fetcher(target_currency, &items, |date, from, to| {
                assert_eq!(from, Currency::GBP);
                assert_eq!(to, target_currency);
                assert_eq!(date, Date::from_str("2025-05-31").unwrap());
                Ok(UnitPrice::from(1.0)) // Mocking the exchange rate to always return 1.0
            });
        let rates = rates.unwrap();
        let rates = rates.rates();
        assert_eq!(rates.len(), 1);
        assert!(rates.contains_key(&Currency::GBP));
    }

    #[test]
    fn test_get_exchange_rate_with_fetcher_when_from_to_is_equal() {
        let date = Date::from_str("2025-04-30").unwrap();
        let from = Currency::EUR;
        let to = Currency::EUR;
        let rate = get_exchange_rate_with_fetcher(date, from, to, |url| {
            assert_eq!(
                url,
                "https://api.frankfurter.app/2025-04-30?from=EUR&to=EUR"
            );
            Ok(Mock { json: "{}" }) // Mocking the fetcher to return an empty response
        });
        assert!(rate.is_ok());
        assert_eq!(rate.unwrap(), UnitPrice::from(1.0));
    }
}
