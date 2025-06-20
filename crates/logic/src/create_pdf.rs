use crate::prelude::*;

/// Compile the Typst source into a PDF and save it at the specified path.
pub fn create_pdf(
    input: ValidInput,
    render: impl Fn(L18n, DataTypstCompat) -> Result<Pdf>,
) -> Result<PathBuf> {
    let data_from_disk = read_data_from_disk()?;
    let l18n = get_localization(input.language())?;
    let data = prepare_invoice_input_data(data_from_disk, input, None)?;
    let output_path = data.absolute_path()?;
    let pdf = render(l18n, data)?;
    save_pdf(pdf, &output_path)?;
    Ok(output_path)
}

/// Saves the PDF file `pdf` to the specified path `pdf_path`.
fn save_pdf(pdf: Pdf, pdf_path: impl AsRef<Path>) -> Result<PathBuf> {
    info!("Saving PDF to: '{}'", pdf_path.as_ref().display());
    // now save the PDF to a file
    let output_path = PathBuf::from(pdf_path.as_ref());
    std::fs::write(&output_path, pdf.as_ref()).map_err(|e| {
        let msg = format!("Failed to write PDF to {}: {}", output_path.display(), e);
        error!("{}", msg);
        Error::SavePdf { underlying: msg }
    })?;
    info!("✅ Saved PDF to: '{}'", pdf_path.as_ref().display());
    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use test_log::test;

    #[test]
    fn test_create_pdf() {
        let out = NamedTempFile::new().unwrap().path().to_path_buf();
        let input = ValidInput::builder()
            .maybe_output_path(out.clone())
            .month(YearAndMonth::sample())
            .build();
        let dummy_pdf_data = Vec::from(b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog >>\nendobj\n");
        let path = create_pdf(input, |_, _| {
            // Simulate PDF rendering
            Ok(Pdf::from(dummy_pdf_data.clone()))
        })
        .unwrap();
        assert_eq!(path, out);
        let result = std::fs::read(&path).unwrap();
        assert_eq!(result, dummy_pdf_data);
    }

    #[test]
    fn test_save_pdf() {
        let tmp_file = NamedTempFile::new().unwrap();
        let tmp_file_path = tmp_file.path();
        let dummy_pdf_bytes = Vec::from(b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog >>\nendobj\n");
        let dummy_pdf = Pdf::from(dummy_pdf_bytes.clone());
        let result = save_pdf(dummy_pdf, tmp_file_path);
        assert!(result.is_ok());
        assert!(tmp_file_path.exists());
        assert_eq!(std::fs::read(tmp_file_path).unwrap(), dummy_pdf_bytes);
    }
}
