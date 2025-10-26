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
pub fn create_pdf_with_data<Period: IsPeriod>(
    data: Data<Period>,
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
    let prepared_data = data.clone();
    let pdf = render(l18n, data, layout)?;
    save_pdf(pdf.clone(), &output_path).map_err(|underlying| Error::SavePdf { underlying })?;
    Ok(NamedPdf::builder()
        .pdf(pdf)
        .saved_at(output_path.clone())
        .name(name)
        .prepared_data(prepared_data)
        .build())
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
            .period(YearMonthAndFortnight::sample())
            .build();
        let dummy_pdf_data = Vec::from(b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog >>\nendobj\n");
        let named_pdf = create_pdf_with_data::<YearAndMonth>(Data::sample(), input, |_, _, _| {
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
