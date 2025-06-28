use crate::{logic::prepare_data::_get_exchange_rate_with_fetcher, prelude::*};

/// Makes blocking requests to the Frankfurter API to get the exchange rate
pub(super) fn get_exchange_rate(date: &Date, from: Currency, to: Currency) -> Result<UnitPrice> {
    _get_exchange_rate_with_fetcher(*date, from, to, |url| {
        reqwest::blocking::get(&url).map_err(|e| Error::NetworkError {
            underlying: format!("Fetch exchange rate {}: {}", url, e),
        })
    })
}
