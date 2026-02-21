use klirr_foundation::{FontRequiring, Pdf, ToTypstFn};
use klirr_render_pdf::{DocumentPlan, Error as RenderPdfError, InlineModule, render_document};

pub const TYPST_VIRTUAL_NAME_MAIN: &str = "main.typ";
pub const TYPST_VIRTUAL_NAME_LAYOUT: &str = "layout.typ";
pub const TYPST_VIRTUAL_NAME_DATA: &str = "data.typ";
pub const TYPST_VIRTUAL_NAME_L10N: &str = "l10n.typ";
pub const TYPST_FOUNDATION_NAME: &str = "foundation.typ";
pub const TYPST_FOUNDATION_CONTENT: &str = include_str!("../../foundation/layout/foundation.typ");

/// Renders a PDF document using Typst with the provided layout, localization, and data.
pub fn render<I: ToTypstFn, D: ToTypstFn, L: ToTypstFn + FontRequiring, E>(
    i18n: I,
    data: D,
    layout: L,
    map_render_error: impl Fn(RenderPdfError) -> E,
) -> Result<Pdf, E> {
    let l10n_typst_str = i18n.to_typst_fn();
    let data_typst_str = data.to_typst_fn();
    let layout_typst_str = layout.to_typst_fn();
    let main = format!(
        r#"
    #import "{}": provide as provide_data
    #import "{}": provide as provide_localization
    #import "{}": render
    #render(provide_data(), provide_localization())
    "#,
        TYPST_VIRTUAL_NAME_DATA, TYPST_VIRTUAL_NAME_L10N, TYPST_VIRTUAL_NAME_LAYOUT
    );
    let plan = DocumentPlan::new(
        layout.required_fonts(),
        InlineModule::new(TYPST_VIRTUAL_NAME_MAIN, main),
    )
    .with_modules(vec![
        InlineModule::new(TYPST_FOUNDATION_NAME, TYPST_FOUNDATION_CONTENT),
        InlineModule::new(TYPST_VIRTUAL_NAME_LAYOUT, layout_typst_str),
        InlineModule::new(TYPST_VIRTUAL_NAME_L10N, l10n_typst_str),
        InlineModule::new(TYPST_VIRTUAL_NAME_DATA, data_typst_str),
    ]);

    render_document(&plan).map_err(map_render_error)
}

#[cfg(test)]
mod tests {
    use crate::render_test_helpers::*;
    use klirr_core_invoice::{
        Currency, Data, Date, ExchangeRatesMap, HasSample, InvoicedItems, Language, UnitPrice,
        ValidInput,
    };
    use test_log::test;

    #[test]
    fn sample_expenses() {
        if running_in_ci() {
            // Skip this test in CI, as it requires imagemagick to be installed.
            return;
        }
        compare_image_against_expected(
            Data::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Expenses)
                .date("2025-05-31".parse::<Date>().unwrap())
                .language(Language::EN)
                .build(),
            fixture("expected_expenses.png"),
            MockedExchangeRatesFetcher::from(ExchangeRatesMap::from_iter([
                (Currency::EUR, UnitPrice::from(10)),
                (Currency::SEK, UnitPrice::from(10)),
            ])),
        );
    }

    #[test]
    fn sample_services() {
        if running_in_ci() {
            // Skip this test in CI, as it requires imagemagick to be installed.
            return;
        }
        compare_image_against_expected(
            Data::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Service { time_off: None })
                .date(Date::sample())
                .language(Language::EN)
                .build(),
            fixture("expected_services.png"),
            MockedExchangeRatesFetcher::default(),
        );
    }
}
