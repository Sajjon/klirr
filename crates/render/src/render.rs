use crate::prelude::*;
use invoice_typst_logic::prelude::to_typst_let;
use typst::layout::PagedDocument;
use typst_pdf::PdfOptions;
use typst_pdf::pdf;

/// Renders a PDF document using Typst with the provided layout, localization, and data.
pub fn render(layout: impl AsRef<Path>, l18n: L18n, data: DataTypstCompat) -> Result<Pdf> {
    let l18n_typst_str = to_typst_let(&l18n.content());
    let data_typst_str = to_typst_let(&data);
    let bytecount = data_typst_str.len();

    debug!("☑️ Creating typst 'World' (environment/context), this usually takes ~2 seconds.");
    let context = TypstContext::with_path(layout.as_ref(), l18n_typst_str, data_typst_str)?;
    debug!("✅ Created typst 'World' (environment/context)");

    debug!("☑️ Compiling: {:?} - #{} bytes", layout.as_ref(), bytecount);
    let compile_result = typst::compile::<PagedDocument>(&context);
    let doc = compile_result.output.map_err(|e| {
        let msg = format!("Failed to compile Typst source, because: {:?}", e);
        error!("{}", msg);
        Error::BuildPdf { underlying: msg }
    })?;
    debug!("✅ Compiled typst source: #{} pages", doc.pages.len());
    let export_pdf_options = &PdfOptions::default();
    let pdf_bytes = pdf(&doc, export_pdf_options).map_err(|e| {
        let msg = format!("Failed to export PDF, because: {:?}", e);
        error!("{}", msg);
        Error::ExportDocumentToPdf { underlying: msg }
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

    #[test]
    fn test_render() {
        let mut tempfile = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        let inline_layout = r#"=Test Pdf"#;
        // save inline layout to the temp file
        tempfile
            .write_all(inline_layout.as_bytes())
            .expect("Failed to write to temp file");
        let layout_path = tempfile.path();
        let l18n = L18n::new(Language::EN).unwrap();
        let data = DataTypstCompat::sample();

        let pdf_result = render(layout_path, l18n, data).unwrap();
        assert!(!pdf_result.as_ref().is_empty());
    }

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
