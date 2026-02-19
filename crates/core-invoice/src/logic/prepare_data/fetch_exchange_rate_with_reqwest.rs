use crate::{
    Currency, Date, Error, Result, UnitPrice, logic::prepare_data::_get_exchange_rate_with_fetcher,
};

/// Makes blocking requests to the Frankfurter API to get the exchange rate
pub(super) fn get_exchange_rate(date: &Date, from: Currency, to: Currency) -> Result<UnitPrice> {
    _get_exchange_rate_with_fetcher(*date, from, to, |url| {
        reqwest::blocking::get(&url)
            .map_err(Error::network_error(format!("Fetch exchange rate {url}")))
    })
}
