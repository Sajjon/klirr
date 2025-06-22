use crate::prelude::*;
use invoice_typst_logic::prelude::to_typst_let;
use typst::layout::PagedDocument;
use typst_pdf::PdfOptions;
use typst_pdf::pdf;

pub const TYPST_VIRTUAL_NAME_MAIN: &str = "main.typ";
pub const TYPST_VIRTUAL_NAME_LAYOUT: &str = "layout.typ";
pub const TYPST_VIRTUAL_NAME_DATA: &str = "data.typ";
pub const TYPST_VIRTUAL_NAME_L18N: &str = "l18n.typ";

const INVOICE_TYP: &str = include_str!("../../../crates/render/src/invoice.typ");

/// Renders a PDF document using Typst with the provided layout, localization, and data.
pub fn render(l18n: L18n, data: DataTypstCompat) -> Result<Pdf> {
    let l18n_typst_str = to_typst_let(&l18n.content());
    let data_typst_str = to_typst_let(&data);
    let main = format!(
        r#"
    #import "{}": provide as provide_data
    #import "{}": provide as provide_localization
    #import "{}": render_invoice
    #render_invoice(provide_data(), provide_localization())
    "#,
        TYPST_VIRTUAL_NAME_DATA, TYPST_VIRTUAL_NAME_L18N, TYPST_VIRTUAL_NAME_LAYOUT
    );

    debug!("☑️ Creating typst 'World' (environment/context), this usually takes ~2 seconds.");
    let context = TypstContext::with_inline(main, INVOICE_TYP.to_owned(), l18n_typst_str, data_typst_str)?;
    debug!("✅ Created typst 'World' (environment/context)");

    debug!("☑️ Compiling typst...");
    let compile_result = typst::compile::<PagedDocument>(&context);
    let doc = compile_result.output.map_err(|e| Error::BuildPdf {
        underlying: format!("{:?}", e),
    })?;
    debug!("✅ Compiled typst source: #{} pages", doc.pages.len());
    let export_pdf_options = &PdfOptions::default();
    let pdf_bytes = pdf(&doc, export_pdf_options).map_err(|e| Error::ExportDocumentToPdf {
        underlying: format!("{:?}", e),
    })?;
    // Convert the exported PDF bytes into a Pdf type.
    let pdf = Pdf::from(pdf_bytes);
    Ok(pdf)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::render_test_helpers::*;

    use super::*;
    use test_log::test;

    // #[test]
    // fn test_render() {
    //     let mut tempfile = tempfile::NamedTempFile::new().expect("Failed to create temp file");
    //     let inline_layout = r#"=Test Pdf"#;
    //     // save inline layout to the temp file
    //     tempfile
    //         .write_all(inline_layout.as_bytes())
    //         .expect("Failed to write to temp file");
    //     let layout_path = tempfile.path();
    //     let l18n = L18n::new(Language::EN).unwrap();
    //     let data = DataTypstCompat::sample();

    //     let pdf_result = render(layout_path, l18n, data).unwrap();
    //     assert!(!pdf_result.as_ref().is_empty());
    // }

    #[test]
    fn sample_expenses() {
        if running_in_ci() {
            // Skip this test in CI, as it requires imagemagick to be installed.
            return;
        }
        compare_image_against_expected(
            DataFromDisk::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Expenses)
                .month(YearAndMonth::sample())
                .language(Language::EN)
                .build(),
            fixture("expected_expenses.png"),
        );
    }

    #[test]
    fn sample_services() {
        if running_in_ci() {
            // Skip this test in CI, as it requires imagemagick to be installed.
            return;
        }
        compare_image_against_expected(
            DataFromDisk::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Service { days_off: None })
                .month(YearAndMonth::sample())
                .language(Language::EN)
                .build(),
            fixture("expected_services.png"),
        );
    }
}
