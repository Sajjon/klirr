use std::{borrow::Borrow, collections::HashMap, path::PathBuf};

use bon::Builder;
use chrono::NaiveDate;
use getset::Getters;
use indexmap::IndexMap;
use log::{debug, warn};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{data_dir, deserialize_contents_of_ron, path_to_ron_file_with_base, save_to_disk};

pub type Result<T, E = ExchangeRatesError> = std::result::Result<T, E>;

const FRANKFURTER_API: &str = "https://api.frankfurter.app";
const CACHED_RATES_FILE_NAME: &str = "cached_rates";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExchangeRatesError {
    NetworkError { underlying: String },
    ParseError { underlying: String },
    MissingRate { target: String, base: String },
}

impl ExchangeRatesError {
    pub fn parse_error(underlying: impl std::fmt::Display) -> Self {
        Self::ParseError {
            underlying: underlying.to_string(),
        }
    }

    pub fn network_error(underlying: impl std::fmt::Display) -> Self {
        Self::NetworkError {
            underlying: underlying.to_string(),
        }
    }
}

impl std::fmt::Display for ExchangeRatesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError { underlying } => {
                write!(
                    f,
                    "Failed fetch exchange rate from API, because: {underlying}"
                )
            }
            Self::ParseError { underlying } => {
                write!(
                    f,
                    "Failed to parse exchange rate response, because: {underlying}"
                )
            }
            Self::MissingRate { target, base } => {
                write!(f, "Found no exchange rate for {target} based on {base}")
            }
        }
    }
}

impl std::error::Error for ExchangeRatesError {}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder, Getters)]
pub struct ExchangeRateItem {
    #[getset(get = "pub")]
    transaction_date: NaiveDate,
    #[builder(into)]
    #[getset(get = "pub")]
    source_currency: String,
}

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
    rates: HashMap<String, Decimal>,
}

pub trait DeserializableResponse {
    fn json<T: serde::de::DeserializeOwned>(self) -> Result<T>;
}

impl DeserializableResponse for reqwest::blocking::Response {
    fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        self.json().map_err(ExchangeRatesError::parse_error)
    }
}

/// Formats a URL for the [Frankfurter API][api] to fetch exchange rates
///
/// [api]: https://frankfurter.dev/
fn format_url(date: NaiveDate, from: &str, to: &str) -> String {
    format!("{FRANKFURTER_API}/{date}?from={from}&to={to}")
}

/// Makes blocking requests to the [Frankfurter API][api] to get the exchange rate
///  
/// [api]: https://frankfurter.dev/
pub fn get_exchange_rate_with_fetcher<T: DeserializableResponse>(
    date: NaiveDate,
    from: &str,
    to: &str,
    fetcher: impl Fn(String) -> Result<T>,
) -> Result<Decimal> {
    if from == to {
        return Ok(Decimal::ONE);
    }
    debug!("Fetching {from}/{to}@{date} rate.");
    fetcher(format_url(date, from, to))?
        .json::<FrankfurterApiResponse>()
        .and_then(|response| {
            response
                .rates()
                .get(to)
                .cloned()
                .ok_or(ExchangeRatesError::MissingRate {
                    target: to.to_owned(),
                    base: from.to_owned(),
                })
        })
}

pub fn get_exchange_rate_with_reqwest(date: &NaiveDate, from: &str, to: &str) -> Result<Decimal> {
    get_exchange_rate_with_fetcher(*date, from, to, |url| {
        reqwest::blocking::get(&url).map_err(ExchangeRatesError::network_error)
    })
}

pub type ExchangeRatesMap = IndexMap<String, Decimal>;

#[derive(Builder)]
pub struct ExchangeRatesFetcher<T = ()> {
    path_to_cache: PathBuf,
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

type FetchedNew = bool;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
struct CachedRates(IndexMap<NaiveDate, IndexMap<String, IndexMap<String, Decimal>>>);

impl CachedRates {
    fn rates_for_day(
        &mut self,
        date: impl Borrow<NaiveDate>,
    ) -> &mut IndexMap<String, IndexMap<String, Decimal>> {
        self.0.entry(*date.borrow()).or_default()
    }

    fn rates_for_day_and_from_currency(
        &mut self,
        date: impl Borrow<NaiveDate>,
        from: impl AsRef<str>,
    ) -> &mut IndexMap<String, Decimal> {
        self.rates_for_day(date)
            .entry(from.as_ref().to_owned())
            .or_default()
    }

    fn load_else_fetch(
        &mut self,
        date: impl Borrow<NaiveDate>,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        fetch: impl FnOnce(&NaiveDate, &str, &str) -> Result<Decimal>,
    ) -> Result<(Decimal, FetchedNew)> {
        let date = *date.borrow();
        let from = from.as_ref().to_owned();
        let to = to.as_ref().to_owned();
        let rates_on_day_from_source = self.rates_for_day_and_from_currency(date, &from);
        if let Some(rate) = rates_on_day_from_source.get(&to) {
            Ok((*rate, false))
        } else {
            let rate = fetch(&date, &from, &to)?;
            rates_on_day_from_source.insert(to, rate);
            Ok((rate, true))
        }
    }
}

impl<T> ExchangeRatesFetcher<T> {
    fn path(&self) -> PathBuf {
        path_to_ron_file_with_base(&self.path_to_cache, CACHED_RATES_FILE_NAME)
    }

    fn load_cache(&self) -> Result<CachedRates> {
        deserialize_contents_of_ron(self.path())
            .map_err(|error| ExchangeRatesError::parse_error(format!("{error:?}")))
    }

    fn save_cache(&self, rates: &CachedRates) -> Result<()> {
        save_to_disk(rates, self.path())
            .map(|_| ())
            .map_err(|error| ExchangeRatesError::parse_error(format!("{error:?}")))
    }

    fn do_fetch(
        cache: &mut CachedRates,
        target_currency: &str,
        items: Vec<ExchangeRateItem>,
    ) -> Result<(ExchangeRatesMap, FetchedNew)> {
        let mut fetched_new_rates = false;
        let mut rates: ExchangeRatesMap = IndexMap::new();
        for expense in items {
            let date = expense.transaction_date();
            let from = expense.source_currency();
            let to = target_currency;
            let (rate, is_new) =
                cache.load_else_fetch(date, from, to, get_exchange_rate_with_reqwest)?;
            fetched_new_rates |= is_new;
            rates.insert(from.clone(), rate);
        }
        Ok((rates, fetched_new_rates))
    }

    fn load_cache_else_new(&self) -> CachedRates {
        self.load_cache().unwrap_or_else(|_| {
            debug!("No cached exchange rates found, fetching new rates.");
            CachedRates::default()
        })
    }

    fn update_cache_if_needed(&self, rates_by_day: &CachedRates, fetched_new_rates: bool) {
        if !fetched_new_rates {
            debug!("ℹ️ No new rates fetched, used only cached rates.");
            return;
        }
        debug!(
            "☑️ Fetched new rates, updating cache: {}",
            self.path_to_cache.display()
        );
        match self.save_cache(rates_by_day) {
            Ok(_) => debug!("✅ Cached exchange rates updated."),
            Err(e) => {
                warn!("Failed to cache exchange rates: {e} (this has no affect on PDF generation.)")
            }
        }
    }

    pub fn fetch_for_items(
        &self,
        target_currency: impl Into<String>,
        items: Vec<ExchangeRateItem>,
    ) -> Result<ExchangeRatesMap> {
        let target_currency = target_currency.into();
        let mut rates_by_day = self.load_cache_else_new();
        let (rates, fetched_new_rates) =
            Self::do_fetch(&mut rates_by_day, &target_currency, items)?;
        debug!("✅ Fetched exchanges rates for #{} expenses.", rates.len());
        self.update_cache_if_needed(&rates_by_day, fetched_new_rates);
        Ok(rates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_format_url() {
        let date = chrono::NaiveDate::from_ymd_opt(2025, 4, 30).unwrap();
        let url = format_url(date, "GBP", "EUR");
        assert_eq!(
            url,
            "https://api.frankfurter.app/2025-04-30?from=GBP&to=EUR"
        );
    }

    #[test]
    fn no_fetch_needed_when_same_currency() {
        let date = chrono::NaiveDate::from_ymd_opt(2025, 4, 30).unwrap();
        let rate = get_exchange_rate_with_fetcher::<MockResponse>(date, "EUR", "EUR", |_url| {
            unreachable!("fetch should not be called for equal currency");
        })
        .unwrap();
        assert_eq!(rate, Decimal::ONE);
    }

    struct MockResponse(&'static str);
    impl DeserializableResponse for MockResponse {
        fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
            serde_json::from_str(self.0).map_err(ExchangeRatesError::parse_error)
        }
    }

    #[test]
    fn gets_rate_from_mocked_fetcher() {
        let date = chrono::NaiveDate::from_ymd_opt(2025, 4, 30).unwrap();
        let rate = get_exchange_rate_with_fetcher(date, "GBP", "EUR", |_url| {
            Ok::<MockResponse, ExchangeRatesError>(MockResponse(r#"{"rates":{"EUR":"1.174"}}"#))
        })
        .unwrap();
        assert_eq!(rate.to_string(), "1.174");
    }

    #[test]
    fn fetcher_uses_custom_cache_dir() {
        let tempdir = tempdir().unwrap();
        let fetcher = ExchangeRatesFetcher::builder()
            .path_to_cache(tempdir.path().to_path_buf())
            .extra(())
            .build();
        let path = fetcher.path();
        assert!(path.ends_with("cached_rates.ron"));
    }
}
