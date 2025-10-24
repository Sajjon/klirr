use crate::prelude::*;
use klirr_render_pdf::{DocumentPlan, Error as RenderPdfError, InlineModule, render_document};

pub const TYPST_VIRTUAL_NAME_MAIN: &str = "main.typ";
pub const TYPST_VIRTUAL_NAME_LAYOUT: &str = "layout.typ";
pub const TYPST_VIRTUAL_NAME_DATA: &str = "data.typ";
pub const TYPST_VIRTUAL_NAME_L18N: &str = "l18n.typ";

/// Renders a PDF document using Typst with the provided layout, localization, and data.
pub fn render(l18n: L18n, data: PreparedData, layout: Layout) -> Result<Pdf> {
    let l18n_typst_str = l18n.content().to_typst_fn();
    let data_typst_str = data.to_typst_fn();
    let layout_typst_str = layout.to_typst_fn();
    let main = format!(
        r#"
    #import "{}": provide as provide_data
    #import "{}": provide as provide_localization
    #import "{}": render_invoice
    #render_invoice(provide_data(), provide_localization())
    "#,
        TYPST_VIRTUAL_NAME_DATA, TYPST_VIRTUAL_NAME_L18N, TYPST_VIRTUAL_NAME_LAYOUT
    );
    let plan = DocumentPlan::new(
        layout.required_fonts(),
        InlineModule::new(TYPST_VIRTUAL_NAME_MAIN, main),
    )
    .with_modules(vec![
        InlineModule::new(TYPST_VIRTUAL_NAME_LAYOUT, layout_typst_str),
        InlineModule::new(TYPST_VIRTUAL_NAME_L18N, l18n_typst_str),
        InlineModule::new(TYPST_VIRTUAL_NAME_DATA, data_typst_str),
    ]);

    render_document(&plan).map_err(map_render_error)
}

fn map_render_error(error: RenderPdfError) -> Error {
    match error {
        RenderPdfError::LoadSource { underlying } => Error::LoadSource { underlying },
        RenderPdfError::BuildPdf { underlying } => Error::BuildPdf { underlying },
        RenderPdfError::ExportDocumentToPdf { underlying } => {
            Error::ExportDocumentToPdf { underlying }
        }
        RenderPdfError::FailedToLoadFont { family_name } => Error::FailedToLoadFont { family_name },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_test_helpers::*;
    use test_log::test;

    #[test]
    fn sample_expenses() {
        if running_in_ci() {
            // Skip this test in CI, as it requires imagemagick to be installed.
            return;
        }
        compare_image_against_expected(
            Data::<YearAndMonth>::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Expenses)
                .period(
                    YearMonthAndFortnight::builder()
                        .year(2025.into())
                        .month(Month::May)
                        .half(MonthHalf::First)
                        .build(),
                )
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
            Data::<YearAndMonth>::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Service { time_off: None })
                .period(YearMonthAndFortnight::sample())
                .language(Language::EN)
                .build(),
            fixture("expected_services.png"),
            MockedExchangeRatesFetcher::default(),
        );
    }
}
