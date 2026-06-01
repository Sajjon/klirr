use std::path::PathBuf;
use std::str::FromStr;

use bon::Builder;
use indexmap::{IndexMap, IndexSet};
use log::{debug, warn};
use serde::{Deserialize, Serialize};

use crate::{
    BankHolidays, CountryCode, Date, Year, data_dir, deserialize_contents_of_ron,
    path_to_ron_file_with_base, save_to_disk,
};

pub type Result<T, E = BankHolidaysError> = std::result::Result<T, E>;

/// Base URL of the [Nager.Date][api] public holiday API.
///
/// Full endpoint: `{NAGER_API}/{year}/{countryCode}`.
///
/// [api]: https://date.nager.at/
const NAGER_API: &str = "https://date.nager.at/api/v3/PublicHolidays";
const CACHED_HOLIDAYS_FILE_NAME: &str = "cached_holidays";
/// Nager.Date holiday `type` denoting a nationwide public holiday — the only
/// type we deduct from billable days.
const HOLIDAY_TYPE_PUBLIC: &str = "Public";

/// Error fetching or parsing bank holidays from the holiday API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BankHolidaysError {
    /// The HTTP request to the holiday API failed.
    NetworkError {
        /// Underlying transport error description.
        underlying: String,
    },
    /// The holiday API response could not be parsed.
    ParseError {
        /// Underlying parse error description.
        underlying: String,
    },
}

impl BankHolidaysError {
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

impl std::fmt::Display for BankHolidaysError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError { underlying } => {
                write!(
                    f,
                    "Failed to fetch bank holidays from API, because: {underlying}"
                )
            }
            Self::ParseError { underlying } => {
                write!(
                    f,
                    "Failed to parse bank holidays response, because: {underlying}"
                )
            }
        }
    }
}

impl std::error::Error for BankHolidaysError {}

/// Abstraction over an HTTP response that can be deserialized as JSON, allowing
/// tests to inject mock responses without performing network requests.
pub trait DeserializableHolidaysResponse {
    fn json<T: serde::de::DeserializeOwned>(self) -> Result<T>;
}

impl DeserializableHolidaysResponse for reqwest::blocking::Response {
    fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        self.json().map_err(BankHolidaysError::parse_error)
    }
}

/// A single holiday entry as returned by the Nager.Date API. Only the fields we
/// need are modelled; the rest are ignored during deserialization.
#[derive(Debug, Clone, Deserialize)]
struct NagerHoliday {
    /// ISO date, e.g. `"2026-06-06"`.
    date: String,
    /// Holiday categories, e.g. `["Public"]`.
    types: Vec<String>,
}

/// Builds the Nager.Date URL for a given year and country.
fn format_url(year: i32, country: &CountryCode) -> String {
    format!("{NAGER_API}/{year}/{}", country.as_str())
}

/// Keeps only nationwide public holidays and converts them into [`BankHolidays`].
fn parse_public_holidays(raw: Vec<NagerHoliday>) -> Result<BankHolidays> {
    let mut dates = IndexSet::new();
    for holiday in raw {
        let is_public = holiday
            .types
            .iter()
            .any(|holiday_type| holiday_type == HOLIDAY_TYPE_PUBLIC);
        if !is_public {
            continue;
        }
        let date = Date::from_str(&holiday.date).map_err(BankHolidaysError::parse_error)?;
        dates.insert(date);
    }
    Ok(BankHolidays::from(dates))
}

/// Fetches public holidays for a year and country using a custom fetcher
/// closure. The closure receives the request URL and returns a deserializable
/// response.
pub fn get_bank_holidays_with_fetcher<T: DeserializableHolidaysResponse>(
    year: i32,
    country: &CountryCode,
    fetcher: impl Fn(String) -> Result<T>,
) -> Result<BankHolidays> {
    debug!("Fetching bank holidays for {} @ {year}.", country.as_str());
    let raw = fetcher(format_url(year, country))?.json::<Vec<NagerHoliday>>()?;
    parse_public_holidays(raw)
}

/// Fetches public holidays via a blocking `reqwest` request.
pub fn get_bank_holidays_with_reqwest(year: i32, country: &CountryCode) -> Result<BankHolidays> {
    get_bank_holidays_with_fetcher(year, country, |url| {
        reqwest::blocking::get(&url).map_err(BankHolidaysError::network_error)
    })
}

type FetchedNew = bool;

/// On-disk cache of bank holidays, keyed by ISO country code then year. Mirrors
/// the exchange-rate cache so holidays fetched once are reused offline.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
struct CachedHolidays(IndexMap<String, IndexMap<i32, BankHolidays>>);

impl CachedHolidays {
    fn holidays_for_country(&mut self, country: &CountryCode) -> &mut IndexMap<i32, BankHolidays> {
        self.0.entry(country.as_str().to_owned()).or_default()
    }

    fn load_else_fetch(
        &mut self,
        country: &CountryCode,
        year: i32,
        fetch: impl FnOnce(i32, &CountryCode) -> Result<BankHolidays>,
    ) -> Result<(BankHolidays, FetchedNew)> {
        let by_year = self.holidays_for_country(country);
        if let Some(holidays) = by_year.get(&year) {
            Ok((holidays.clone(), false))
        } else {
            let holidays = fetch(year, country)?;
            by_year.insert(year, holidays.clone());
            Ok((holidays, true))
        }
    }
}

/// Fetches bank holidays for the vendor's country, caching results to disk.
#[derive(Builder)]
pub struct BankHolidaysFetcher<T = ()> {
    path_to_cache: PathBuf,
    #[allow(dead_code)]
    extra: T,
}

impl Default for BankHolidaysFetcher {
    fn default() -> Self {
        Self {
            path_to_cache: data_dir(),
            extra: (),
        }
    }
}

impl<T> BankHolidaysFetcher<T> {
    fn path(&self) -> PathBuf {
        path_to_ron_file_with_base(&self.path_to_cache, CACHED_HOLIDAYS_FILE_NAME)
    }

    fn load_cache(&self) -> Result<CachedHolidays> {
        deserialize_contents_of_ron(self.path())
            .map_err(|error| BankHolidaysError::parse_error(format!("{error:?}")))
    }

    fn save_cache(&self, cache: &CachedHolidays) -> Result<()> {
        save_to_disk(cache, self.path())
            .map(|_| ())
            .map_err(|error| BankHolidaysError::parse_error(format!("{error:?}")))
    }

    fn load_cache_else_new(&self) -> CachedHolidays {
        self.load_cache().unwrap_or_else(|_| {
            debug!("No cached bank holidays found, fetching anew.");
            CachedHolidays::default()
        })
    }

    fn update_cache_if_needed(&self, cache: &CachedHolidays, fetched_new: bool) {
        if !fetched_new {
            debug!("ℹ️ No new bank holidays fetched, used only cached holidays.");
            return;
        }
        match self.save_cache(cache) {
            Ok(_) => debug!("✅ Cached bank holidays updated."),
            Err(e) => {
                warn!("Failed to cache bank holidays: {e} (this has no affect on PDF generation.)")
            }
        }
    }

    /// Returns the public holidays for `country` in `year`, using the on-disk
    /// cache when available and fetching from the API on a miss.
    pub fn holidays_for(&self, country: &CountryCode, year: Year) -> Result<BankHolidays> {
        let year = i32::from(*year);
        let mut cache = self.load_cache_else_new();
        let (holidays, fetched_new) =
            cache.load_else_fetch(country, year, get_bank_holidays_with_reqwest)?;
        self.update_cache_if_needed(&cache, fetched_new);
        Ok(holidays)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{TempDir, tempdir};
    use test_log::test;

    fn temp_fetcher(tempdir: &TempDir) -> BankHolidaysFetcher {
        BankHolidaysFetcher::builder()
            .path_to_cache(tempdir.path().to_path_buf())
            .extra(())
            .build()
    }

    fn sweden() -> CountryCode {
        CountryCode::new("SE").unwrap()
    }

    struct MockResponse(&'static str);
    impl DeserializableHolidaysResponse for MockResponse {
        fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
            serde_json::from_str(self.0).map_err(BankHolidaysError::parse_error)
        }
    }

    #[test]
    fn test_format_url() {
        let url = format_url(2026, &sweden());
        assert_eq!(url, "https://date.nager.at/api/v3/PublicHolidays/2026/SE");
    }

    #[test]
    fn parses_only_public_holidays_from_mock() {
        let body = r#"[
            {"date":"2026-01-01","types":["Public"]},
            {"date":"2026-06-06","types":["Public","Bank"]},
            {"date":"2026-12-24","types":["Observance"]}
        ]"#;
        let holidays = get_bank_holidays_with_fetcher(2026, &sweden(), |_url| {
            Ok::<MockResponse, BankHolidaysError>(MockResponse(body))
        })
        .unwrap();
        assert_eq!(holidays.len(), 2);
        assert!(holidays.contains(&Date::from_str("2026-01-01").unwrap()));
        assert!(holidays.contains(&Date::from_str("2026-06-06").unwrap()));
        assert!(!holidays.contains(&Date::from_str("2026-12-24").unwrap()));
    }

    #[test]
    fn parse_error_on_bad_date() {
        let body = r#"[{"date":"not-a-date","types":["Public"]}]"#;
        let result = get_bank_holidays_with_fetcher(2026, &sweden(), |_url| {
            Ok::<MockResponse, BankHolidaysError>(MockResponse(body))
        });
        assert!(matches!(result, Err(BankHolidaysError::ParseError { .. })));
    }

    #[test]
    fn fetcher_uses_custom_cache_dir() {
        let tempdir = tempdir().unwrap();
        let fetcher = temp_fetcher(&tempdir);
        assert!(fetcher.path().ends_with("cached_holidays.ron"));
    }

    #[test]
    fn cache_hit_avoids_fetch() {
        let year = 2026;
        let holidays = BankHolidays::new([Date::from_str("2026-06-06").unwrap()]);
        let mut cache = CachedHolidays::default();
        cache
            .holidays_for_country(&sweden())
            .insert(year, holidays.clone());

        let (loaded, fetched_new) = cache
            .load_else_fetch(&sweden(), year, |_year, _country| {
                unreachable!("must not fetch on cache hit")
            })
            .unwrap();

        assert!(!fetched_new);
        assert_eq!(loaded, holidays);
    }

    #[test]
    fn cache_miss_fetches_and_inserts() {
        let year = 2026;
        let holidays = BankHolidays::new([Date::from_str("2026-06-06").unwrap()]);
        let mut cache = CachedHolidays::default();

        let (loaded, fetched_new) = cache
            .load_else_fetch(&sweden(), year, |_year, _country| Ok(holidays.clone()))
            .unwrap();

        assert!(fetched_new);
        assert_eq!(loaded, holidays);
        // Second lookup is now a hit.
        let (_, fetched_new_again) = cache
            .load_else_fetch(&sweden(), year, |_year, _country| {
                unreachable!("second lookup must hit cache")
            })
            .unwrap();
        assert!(!fetched_new_again);
    }

    #[test]
    fn if_fetched_new_is_false_cache_is_unchanged() {
        let tempdir = tempdir().unwrap();
        let fetcher = temp_fetcher(&tempdir);
        fetcher.update_cache_if_needed(&CachedHolidays::default(), false);
        assert!(
            !fetcher.path().exists(),
            "Cache file should not exist when no new holidays were fetched."
        );
    }

    #[test]
    fn if_fetched_new_is_true_cache_is_written() {
        let tempdir = tempdir().unwrap();
        let fetcher = temp_fetcher(&tempdir);
        let mut cache = CachedHolidays::default();
        cache.holidays_for_country(&sweden()).insert(
            2026,
            BankHolidays::new([Date::from_str("2026-06-06").unwrap()]),
        );

        fetcher.update_cache_if_needed(&cache, true);

        let loaded: CachedHolidays = deserialize_contents_of_ron(fetcher.path()).unwrap();
        assert_eq!(loaded, cache);
    }

    #[test]
    fn gibberish_cache_is_reset() {
        let tempdir = tempdir().unwrap();
        let fetcher = temp_fetcher(&tempdir);
        std::fs::write(fetcher.path(), "gibberish").unwrap();
        assert_eq!(fetcher.load_cache_else_new(), CachedHolidays::default());
    }
}
