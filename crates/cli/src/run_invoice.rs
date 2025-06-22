use invoice_typst_render::prelude::render;

use crate::prelude::*;

pub(super) fn run(input: InvoiceInput) -> Result<()> {
    let input = input.parsed()?;
    info!("🔮 Starting PDF creation, input: {}...", input);
    let pdf_location = create_pdf(input, |l18n, data| render(l18n, data))?;
    save_pdf_location_to_tmp_file(pdf_location)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::InvoiceInput;

    #[test]
    fn test_run() {
        let tempfile = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        let input = InvoiceInput::parse_from([
            "invoice",
            "--out",
            &format!("{}", tempfile.path().display()),
        ]);
        let result = run(input);
        assert!(result.is_ok(), "Expected run to succeed, got: {:?}", result);
    }
}
