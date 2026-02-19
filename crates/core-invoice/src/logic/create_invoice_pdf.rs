use crate::{
    Data, Error, ExchangeRatesFetcher, IsPeriod, L10n, Layout, NamedPdf, Path, PreparedData,
    Result, ValidInput, create_folder_to_parent_of_path_if_needed, get_localization,
    prepare_invoice_input_data, read_data_from_disk_with_base_path,
};
use klirr_foundation::{Pdf, save_pdf};

/// Compile the Typst source into a PDF and save it at the specified path, by
/// reading data from disk at the provided path and using the provided `ValidInput`.
pub fn create_invoice_pdf_with_data_base_path<E>(
    data_base_path: impl AsRef<Path>,
    input: ValidInput,
    render: impl Fn(L10n, PreparedData, Layout) -> Result<Pdf, E>,
) -> Result<NamedPdf, E>
where
    E: From<Error>,
{
    let data = read_data_from_disk_with_base_path(data_base_path).map_err(E::from)?;
    create_invoice_pdf_with_data(data, input, render)
}

/// Compile the Typst source into a PDF and save it at the specified path, using
/// the provided `Data` and `ValidInput`.
pub fn create_invoice_pdf_with_data<Period: IsPeriod, E>(
    data: Data<Period>,
    input: ValidInput,
    render: impl Fn(L10n, PreparedData, Layout) -> Result<Pdf, E>,
) -> Result<NamedPdf, E>
where
    E: From<Error>,
{
    let l10n: L10n = get_localization(input.language()).map_err(E::from)?;
    let layout = *input.layout();
    let data = prepare_invoice_input_data(data, input, ExchangeRatesFetcher::default())
        .map_err(E::from)?;
    let output_path_and_name = data.absolute_path_and_name().map_err(E::from)?;
    let output_path = output_path_and_name.path().to_owned();
    let name = output_path_and_name.name().to_owned();
    create_folder_to_parent_of_path_if_needed(&output_path).map_err(E::from)?;
    let prepared_data = data.clone();
    let pdf = render(l10n, data, layout)?;
    save_pdf(pdf.clone(), &output_path)
        .map_err(Error::save_pdf)
        .map_err(E::from)?;
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
    use crate::HasSample;
    use crate::{PathBuf, YearAndMonth, YearMonthAndFortnight};

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
        let named_pdf = create_invoice_pdf_with_data::<YearAndMonth, Error>(
            Data::sample(),
            input,
            |_, _, _| {
                // Simulate PDF rendering
                Ok::<Pdf, Error>(Pdf::from(dummy_pdf_data.clone()))
            },
        )
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
