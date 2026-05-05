use crate::{DocumentPlan, Error, InlineModule, Result, typst_context::TypstContext};
use klirr_foundation::{FontRequiring, Pdf, TYPST_LAYOUT_FOUNDATION, ToTypstFn};
use log::debug;
use typst::layout::PagedDocument;
use typst_pdf::PdfOptions;

pub const TYPST_VIRTUAL_NAME_MAIN: &str = "main.typ";
pub const TYPST_VIRTUAL_NAME_LAYOUT: &str = "layout.typ";
pub const TYPST_VIRTUAL_NAME_DATA: &str = "data.typ";
pub const TYPST_VIRTUAL_NAME_L10N: &str = "l10n.typ";
pub const TYPST_FOUNDATION_NAME: &str = "foundation.typ";
pub const TYPST_FOUNDATION_CONTENT: &str = TYPST_LAYOUT_FOUNDATION;

/// Renders a Typst document described by the provided plan into a PDF.
fn render_document(plan: &DocumentPlan) -> Result<Pdf> {
    debug!("☑️ Creating typst context");
    let context = TypstContext::from_plan(plan)?;
    debug!("☑️ Compiling typst...");
    let compile_result = typst::compile::<PagedDocument>(&context);
    let doc = compile_result.output.map_err(Error::build_pdf)?;
    debug!("✅ Compiled typst source: #{} pages", doc.pages.len());
    let pdf_bytes =
        typst_pdf::pdf(&doc, &PdfOptions::default()).map_err(Error::export_document_to_pdf)?;
    Ok(Pdf::from(pdf_bytes))
}

/// Renders a PDF document using Typst with the provided layout, localization, and data.
pub fn render<I: ToTypstFn, D: ToTypstFn, L: ToTypstFn + FontRequiring, E>(
    i18n: I,
    data: D,
    layout: L,
    map_render_error: impl Fn(Error) -> E,
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
    use crate::{DocumentPlan, render::render_document, render_test_helpers::*};
    use klirr_core_invoice::{
        Currency, Data, Date, ExchangeRatesMap, HasSample, InvoicedItems, Language, UnitPrice,
        ValidInput,
    };
    use klirr_foundation::{FontIdentifier, FontWeight};
    use test_log::test;

    #[test]
    fn renders_simple_document() {
        let plan = DocumentPlan::new(
            [FontIdentifier::ComputerModern(FontWeight::Regular)],
            crate::module::InlineModule::new("main.typ", "#box(\"hello\")"),
        );
        assert!(render_document(&plan).is_ok());
    }

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

    /// Compiles the layout with a non-zero VAT rate. We don't compare against a
    /// fixture image (that would require regenerating the PNG and adds churn),
    /// but successful compilation proves the Typst layout accepts the new
    /// `data.payment_info.vat` field and the conditional VAT row.
    #[test]
    fn services_with_vat_renders_without_error() {
        use klirr_core_invoice::{Vat, prepare_invoice_input_data};
        use rust_decimal::dec;

        let configured_vat = Vat::from_percent(dec!(25)).expect("25% is valid");
        let data = Data::sample();
        let payment_info = data.payment_info().clone().with_vat(configured_vat);
        let data = Data::builder()
            .information(data.information().clone())
            .vendor(data.vendor().clone())
            .client(data.client().clone())
            .payment_info(payment_info)
            .service_fees(data.service_fees().clone())
            .expensed_periods(data.expensed_periods().clone())
            .build();

        let input = ValidInput::builder()
            .items(InvoicedItems::Service { time_off: None })
            .date(Date::sample())
            .language(Language::EN)
            .build();

        let layout = *input.layout();
        let prepared =
            prepare_invoice_input_data(data, input, MockedExchangeRatesFetcher::default()).unwrap();

        // Structural check: VAT propagated to the prepared data unchanged.
        assert_eq!(*prepared.payment_info().vat(), configured_vat);

        // Render the actual PDF (this exercises the new VAT branch).
        let pdf = crate::render::render(
            klirr_core_invoice::L10n::new(Language::EN).unwrap(),
            prepared,
            layout,
            |e| panic!("render failed: {e}"),
        )
        .unwrap();
        assert!(!pdf.as_ref().is_empty(), "rendered PDF should be non-empty");
    }

    #[test]
    fn services_with_zero_vat_renders_without_error() {
        use klirr_core_invoice::prepare_invoice_input_data;

        let input = ValidInput::builder()
            .items(InvoicedItems::Service { time_off: None })
            .date(Date::sample())
            .language(Language::EN)
            .build();
        let layout = *input.layout();
        let prepared = prepare_invoice_input_data(
            Data::sample(),
            input,
            MockedExchangeRatesFetcher::default(),
        )
        .unwrap();

        let pdf = crate::render::render(
            klirr_core_invoice::L10n::new(Language::EN).unwrap(),
            prepared,
            layout,
            |e| panic!("render failed: {e}"),
        )
        .unwrap();
        assert!(!pdf.as_ref().is_empty(), "rendered PDF should be non-empty");
    }

    /// Multi-line invoices (typical for expenses) should still render
    /// successfully when VAT is configured, exercising the Subtotal-row
    /// branch of the layout that single-line service invoices skip.
    #[test]
    fn expenses_with_vat_renders_without_error() {
        use klirr_core_invoice::{Currency, Vat, prepare_invoice_input_data};
        use rust_decimal::dec;

        let configured_vat = Vat::from_percent(dec!(25)).expect("25% is valid");
        let base = Data::sample();
        let payment_info = base.payment_info().clone().with_vat(configured_vat);
        let data = Data::builder()
            .information(base.information().clone())
            .vendor(base.vendor().clone())
            .client(base.client().clone())
            .payment_info(payment_info)
            .service_fees(base.service_fees().clone())
            .expensed_periods(base.expensed_periods().clone())
            .build();

        let input = ValidInput::builder()
            .items(InvoicedItems::Expenses)
            .date("2025-05-31".parse::<Date>().unwrap())
            .language(Language::EN)
            .build();
        let layout = *input.layout();
        let prepared = prepare_invoice_input_data(
            data,
            input,
            MockedExchangeRatesFetcher::from(ExchangeRatesMap::from_iter([
                (Currency::EUR, UnitPrice::from(10)),
                (Currency::SEK, UnitPrice::from(10)),
            ])),
        )
        .unwrap();
        assert_eq!(*prepared.payment_info().vat(), configured_vat);

        let pdf = crate::render::render(
            klirr_core_invoice::L10n::new(Language::EN).unwrap(),
            prepared,
            layout,
            |e| panic!("render failed: {e}"),
        )
        .unwrap();
        assert!(!pdf.as_ref().is_empty(), "rendered PDF should be non-empty");
    }
}
