use crate::prelude::*;

/// Compile the Typst source into a PDF and save it at the specified path, by
/// reading data from disk at the provided path and using the provided `ValidInput`.
pub fn create_pdf_with_data_base_path(
    data_base_path: impl AsRef<Path>,
    input: ValidInput,
    render: impl Fn(L18n, PreparedData, Layout) -> Result<Pdf>,
) -> Result<NamedPdf> {
    let data = read_data_from_disk_with_base_path(data_base_path)?;
    create_pdf_with_data(data, input, render)
}

/// Compile the Typst source into a PDF and save it at the specified path, using
/// the provided `Data` and `ValidInput`.
pub fn create_pdf_with_data(
    data: Data,
    input: ValidInput,
    render: impl Fn(L18n, PreparedData, Layout) -> Result<Pdf>,
) -> Result<NamedPdf> {
    let l18n: L18n = get_localization(input.language())?;
    let layout = *input.layout();
    let data = prepare_invoice_input_data(data, input, ExchangeRatesFetcher::default())?;
    let output_path_and_name = data.absolute_path_and_name()?;
    let output_path = output_path_and_name.path().to_owned();
    let name = output_path_and_name.name().to_owned();
    create_folder_to_parent_of_path_if_needed(&output_path)?;
    let pdf = render(l18n, data, layout)?;
    save_pdf(pdf.clone(), &output_path)?;
    Ok(NamedPdf::builder()
        .pdf(pdf)
        .saved_at(output_path.clone())
        .name(name)
        .build())
}

/// Saves the PDF file `pdf` to the specified path `pdf_path`.
fn save_pdf(pdf: Pdf, pdf_path: impl AsRef<Path>) -> Result<PathBuf> {
    info!("Saving PDF to: '{}'", pdf_path.as_ref().display());
    // now save the PDF to a file
    let output_path = PathBuf::from(pdf_path.as_ref());
    std::fs::write(&output_path, pdf.as_ref()).map_err(|e| Error::SavePdf {
        underlying: format!("Write PDF to {}: {}", output_path.display(), e),
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
        let named_pdf = create_pdf_with_data(Data::sample(), input, |_, _, _| {
            // Simulate PDF rendering
            Ok(Pdf::from(dummy_pdf_data.clone()))
        })
        .unwrap();
        assert_eq!(named_pdf.saved_at(), &out);
        let result = std::fs::read(named_pdf.saved_at()).unwrap();
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

    #[test]
    fn test_save_pdf_invalid_path() {
        let invalid_path = PathBuf::from("/invalid/path/to/save.pdf");
        let dummy_pdf = Pdf::from(Vec::from(
            b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog >>\nendobj\n",
        ));
        let result = save_pdf(dummy_pdf, &invalid_path);
        assert!(result.is_err(), "Expected save to fail, got: {:?}", result);
    }
}
