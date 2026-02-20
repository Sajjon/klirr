use insta::assert_snapshot;
use klirr_core_invoice::{
    Currency, Data, ExchangeRates, ExchangeRatesMap, HasSample, InvoicedItems, L10n, Language,
    PreparedData, UnitPrice, ValidInput, YearAndMonth,
};
use klirr_foundation::ToTypstFn;
use rust_decimal::dec;

fn sample_exchange_rates() -> ExchangeRates {
    ExchangeRates::builder()
        .target_currency(Currency::EUR)
        .rates(ExchangeRatesMap::from_iter([
            (Currency::GBP, UnitPrice::from(dec!(1.174))),
            (Currency::SEK, UnitPrice::from(dec!(11.05))),
        ]))
        .build()
}

fn prepared_data_from(input: ValidInput) -> PreparedData {
    Data::<YearAndMonth>::sample()
        .to_partial(input)
        .unwrap()
        .to_typst(sample_exchange_rates())
        .unwrap()
}

#[test]
fn data_expenses_to_typst() {
    let input = ValidInput::builder()
        .items(InvoicedItems::Expenses)
        .period(YearAndMonth::may(2025).into())
        .language(Language::EN)
        .build();
    let typst = prepared_data_from(input).to_typst_fn();
    assert_snapshot!("data_expenses_to_typst", typst);
}

#[test]
fn data_services_to_typst() {
    let input = ValidInput::builder()
        .items(InvoicedItems::Service { time_off: None })
        .period(YearAndMonth::may(2025).into())
        .language(Language::EN)
        .build();
    let typst = prepared_data_from(input).to_typst_fn();
    assert_snapshot!("data_services_to_typst", typst);
}

#[test]
fn l10n_english_to_typst() {
    let typst = L10n::new(Language::EN).unwrap().content().to_typst_fn();
    assert_snapshot!("l10n_english_to_typst", typst);
}
