use crate::{
    error::{Error, Result},
    module::DocumentPlan,
    typst_context::TypstContext,
};
use klirr_foundation::Pdf;
use log::debug;
use typst::layout::PagedDocument;
use typst_pdf::{PdfOptions, pdf};

/// Renders a Typst document described by the provided plan into a PDF.
pub fn render_document(plan: &DocumentPlan) -> Result<Pdf> {
    debug!("☑️ Creating typst context");
    let context = TypstContext::from_plan(plan)?;
    debug!("☑️ Compiling typst...");
    let compile_result = typst::compile::<PagedDocument>(&context);
    let doc = compile_result.output.map_err(Error::build_pdf)?;
    debug!("✅ Compiled typst source: #{} pages", doc.pages.len());
    let pdf_bytes = pdf(&doc, &PdfOptions::default()).map_err(Error::export_document_to_pdf)?;
    Ok(Pdf::from(pdf_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
