use std::{borrow::Borrow, collections::HashMap};

use crate::{logic::prepare_data::fetch_exchange_rate_with_reqwest::get_exchange_rate, prelude::*};

const FRANKFURTER_API: &str = "https://api.frankfurter.app";

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

pub(super) trait DeserializableResponse {
    fn json<T: serde::de::DeserializeOwned>(self) -> Result<T>;
}
impl DeserializableResponse for reqwest::blocking::Response {
    fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        self.json().map_err(|e| Error::ParseError {
            underlying: format!("Parse JSON: {}", e),
        })
    }
}

fn format_url(date: Date, from: Currency, to: Currency) -> String {
    format!("{}/{}?from={}&to={}", FRANKFURTER_API, date, from, to)
}

/// Makes blocking requests to the Frankfurter API to get the exchange rate
pub(super) fn _get_exchange_rate_with_fetcher<T: DeserializableResponse>(
    date: Date,
    from: Currency,
    to: Currency,
    fetcher: impl Fn(String) -> Result<T>,
) -> Result<UnitPrice> {
    if from == to {
        return Ok(UnitPrice::from(1.0));
    }
    debug!("Fetching {}/{}@{} rate.", from, to, date);
    fetcher(format_url(date, from, to))?
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

pub type ExchangeRatesMap = IndexMap<Currency, UnitPrice>;

// fn get_exchange_rates_if_needed(
//     target_currency: Currency,
//     items: &LineItemsPricedInSourceCurrency,
// ) -> Result<ExchangeRates> {
//     get_exchange_rates_if_needed_with_fetcher(target_currency, items, get_exchange_rate)
// }

#[derive(TypedBuilder)]
pub struct ExchangeRatesFetcher<T = ()> {
    path_to_cache: PathBuf,
    /// Useful for testing, allows to use a temporary directory for caching
    #[allow(dead_code)]
    extra: T,
}

impl Default for ExchangeRatesFetcher {
    fn default() -> Self {
        Self {
            path_to_cache: data_dir(),
            extra: (),
        }
    }
}

type FromCurrency = Currency;
type ToCurrency = Currency;
type ExchangeRate = UnitPrice;

type FetchedNew = bool;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct CachedRates(IndexMap<Date, IndexMap<FromCurrency, IndexMap<ToCurrency, ExchangeRate>>>);
impl CachedRates {
    fn _rates_for_day(
        &mut self,
        date: impl Borrow<Date>,
    ) -> &mut IndexMap<FromCurrency, IndexMap<ToCurrency, ExchangeRate>> {
        self.0.entry(*date.borrow()).or_default()
    }

    fn _rates_for_day_and_from_currency(
        &mut self,
        date: impl Borrow<Date>,
        from: impl Borrow<FromCurrency>,
    ) -> &mut IndexMap<ToCurrency, ExchangeRate> {
        self._rates_for_day(date).entry(*from.borrow()).or_default()
    }

    fn load_else_fetch(
        &mut self,
        date: impl Borrow<Date>,
        from: impl Borrow<FromCurrency>,
        to: impl Borrow<ToCurrency>,
        fetch: impl FnOnce(&Date, FromCurrency, ToCurrency) -> Result<ExchangeRate>,
    ) -> Result<(ExchangeRate, FetchedNew)> {
        let date = *date.borrow();
        let from = *from.borrow();
        let to = *to.borrow();
        let rates_on_day_from_source = self._rates_for_day_and_from_currency(date, from);

        if let Some(rate) = rates_on_day_from_source.get(&to) {
            Ok((*rate, false))
        } else {
            let rate = fetch(&date, from, to)?;
            rates_on_day_from_source.insert(to, rate);
            Ok((rate, true))
        }
    }
}

impl<T> ExchangeRatesFetcher<T> {
    fn load_cache(&self) -> Result<CachedRates> {
        load_data(&self.path_to_cache, DATA_FILE_NAME_CACHED_RATES)
    }
    fn save_cache(&self, rates: &CachedRates) -> Result<()> {
        save_to_disk(
            rates,
            path_to_ron_file_with_base(&self.path_to_cache, DATA_FILE_NAME_CACHED_RATES),
        )
    }
}

impl<T> FetchExchangeRates for ExchangeRatesFetcher<T> {
    fn fetch_for_items(
        &self,
        target_currency: Currency,
        items: Vec<Item>,
    ) -> Result<ExchangeRates> {
        let mut rates_by_day = self.load_cache().unwrap_or_else(|_| {
            debug!("No cached exchange rates found, fetching new rates.");
            CachedRates::default()
        });
        let mut fetched_new_rates = false;
        let mut rates: ExchangeRatesMap = IndexMap::new();
        for expense in items {
            let date = expense.transaction_date();
            let from = *expense.currency();
            let to = target_currency;
            let (rate, is_new) = rates_by_day.load_else_fetch(date, from, to, get_exchange_rate)?;
            fetched_new_rates |= is_new;
            rates.insert(from, rate);
        }
        debug!("✅ Fetched exchanges rates for #{} expenses.", rates.len());
        if fetched_new_rates {
            // Update cache
            debug!(
                "✅ Fetched new rates, updating cache: {}",
                self.path_to_cache.display()
            );
            match self.save_cache(&rates_by_day) {
                Ok(_) => debug!("ℹ️ Cached exchange rates updated."),
                Err(e) => {
                    // Failing to update cache is not critical, but we log it
                    // so that the user is aware of it.
                    // They can still use the fetched rates, but they won't be cached.
                    warn!(
                        "Failed to cached exchange rates: {} (this has no affect on PDF generation.)",
                        e
                    );
                }
            }
        } else {
            debug!("ℹ️ No new rates fetched, used only cached rates.");
        }
        let rates = ExchangeRates::builder()
            .target_currency(target_currency)
            .rates(rates)
            .build();
        Ok(rates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    use httpmock::Method::GET;
    use httpmock::MockServer;

    #[derive(Debug, Deserialize, PartialEq)]
    struct MyData {
        name: String,
        age: u32,
    }

    #[test]
    fn test_format_url() {
        let date = Date::from_str("2025-04-30").unwrap();
        let from = Currency::GBP;
        let to = Currency::EUR;
        let url = format_url(date, from, to);
        assert_eq!(
            url,
            "https://api.frankfurter.app/2025-04-30?from=GBP&to=EUR"
        );
    }

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
            serde_json::from_str(self.json).map_err(|e| Error::ParseError {
                underlying: format!("Parse mock response JSON: {}", e),
            })
        }
    }

    #[test]
    fn test_get_exchange_rate() {
        let date = Date::from_str("2025-04-30").unwrap();
        let from = Currency::GBP;
        let to = Currency::EUR;
        let rate = _get_exchange_rate_with_fetcher(date, from, to, |url| {
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
    fn test_successful_deserialization() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/test");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(r#"{"name": "Alice", "age": 30}"#);
        });

        let response = reqwest::blocking::get(format!("{}/test", server.base_url())).unwrap();

        let result: Result<MyData, _> = DeserializableResponse::json(response); // our trait method

        assert_eq!(
            result.unwrap(),
            MyData {
                name: "Alice".to_string(),
                age: 30
            }
        );

        mock.assert();
    }

    #[test]
    fn test_json_parse_error() {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(GET).path("/badjson");
            then.status(200)
                .header("Content-Type", "application/json")
                .body("invalid json");
        });

        let response = reqwest::blocking::get(format!("{}/badjson", server.url("/rates"))).unwrap();

        let result: Result<MyData, _> = DeserializableResponse::json(response);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_exchange_rate_with_fetcher_when_from_to_is_equal() {
        let date = Date::from_str("2025-04-30").unwrap();
        let from = Currency::EUR;
        let to = Currency::EUR;
        let rate = _get_exchange_rate_with_fetcher(date, from, to, |url| {
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
