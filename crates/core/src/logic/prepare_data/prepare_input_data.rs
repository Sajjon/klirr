use crate::prelude::*;

pub trait FetchExchangeRates {
    fn fetch_for_items(&self, target_currency: Currency, items: Vec<Item>)
    -> Result<ExchangeRates>;

    fn fetch_for_line_items(
        &self,
        target_currency: Currency,
        line_items: &LineItemsPricedInSourceCurrency,
    ) -> Result<ExchangeRates> {
        let Ok(expenses) = line_items.clone().try_unwrap_expenses() else {
            debug!("No expenses found, skipping exchange rate fetching.");
            return Ok(ExchangeRates::builder()
                .target_currency(target_currency)
                .rates(ExchangeRatesMap::new())
                .build());
        };
        debug!("☑️ Fetching rates for #{} expenses...", expenses.len());
        self.fetch_for_items(target_currency, expenses)
    }
}

pub fn prepare_invoice_input_data(
    data: Data,
    input: ValidInput,
    exchange_rates_fetcher: impl FetchExchangeRates,
) -> Result<PreparedData> {
    info!("Preparing invoice input data for PDF generation...");
    let partial = data.to_partial(input)?;
    let exchange_rates = exchange_rates_fetcher
        .fetch_for_line_items(*partial.payment_info().currency(), partial.line_items())?;
    let data_typst_compat = partial.to_typst(exchange_rates)?;
    info!("✅ Prepared invoice input data for PDF generation.");
    Ok(data_typst_compat)
}
