use insta::assert_snapshot;
use klirr_core_invoice::{
    Currency, Data, Date, ExchangeRates, ExchangeRatesMap, HasSample, InvoicedItems, L10n,
    Language, PreparedData, UnitPrice, ValidInput, Vat,
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
    Data::sample()
        .to_partial(input)
        .unwrap()
        .to_typst(sample_exchange_rates())
        .unwrap()
}

#[test]
fn data_expenses_to_typst() {
    let input = ValidInput::builder()
        .items(InvoicedItems::Expenses)
        .date("2025-05-31".parse::<Date>().unwrap())
        .language(Language::EN)
        .build();
    let typst = prepared_data_from(input).to_typst_fn();
    assert_snapshot!("data_expenses_to_typst", typst);
}

#[test]
fn data_services_to_typst() {
    let input = ValidInput::builder()
        .items(InvoicedItems::Service { time_off: None })
        .date("2025-05-31".parse::<Date>().unwrap())
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

#[test]
fn data_services_with_vat_to_typst() {
    let input = ValidInput::builder()
        .items(InvoicedItems::Service { time_off: None })
        .date("2025-05-31".parse::<Date>().unwrap())
        .language(Language::EN)
        .build();
    let data = Data::sample();
    let payment_info_with_vat = data
        .payment_info()
        .clone()
        .with_vat(Vat::from_percent(dec!(25)).expect("25% is valid"));
    let data = Data::builder()
        .information(data.information().clone())
        .vendor(data.vendor().clone())
        .client(data.client().clone())
        .payment_info(payment_info_with_vat)
        .service_fees(data.service_fees().clone())
        .expensed_periods(data.expensed_periods().clone())
        .build();
    let typst = data
        .to_partial(input)
        .unwrap()
        .to_typst(sample_exchange_rates())
        .unwrap()
        .to_typst_fn();
    assert_snapshot!("data_services_with_vat_to_typst", typst);
}
