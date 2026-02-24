use crate::{
    AbstractNamedPdf, OutputPath, PathAndName, Pdf, create_folder_to_parent_of_path_if_needed,
    save_pdf,
};

/// Resolves an `OutputPath` to an absolute/usable file path and filename.
pub fn resolve_output_path_and_name(
    output_path: &OutputPath,
    default_folder_name_in_home_dir: &str,
) -> Result<PathAndName, String> {
    match output_path {
        OutputPath::AbsolutePath(path) => {
            let name = path
                .file_name()
                .ok_or_else(|| format!("Output path '{}' has no file name", path.display()))?
                .to_string_lossy()
                .into_owned();
            Ok(PathAndName::builder().path(path.clone()).name(name).build())
        }
        OutputPath::Name(name) => {
            let mut path = dirs_next::home_dir()
                .ok_or_else(|| "Failed to find output dir (home dir)".to_owned())?;
            path.push(default_folder_name_in_home_dir);
            path.push(name);
            Ok(PathAndName::builder().path(path).name(name.clone()).build())
        }
    }
}

/// Renders, saves and returns a named PDF document with its prepared data.
pub fn render_and_save_named_pdf<D, E>(
    prepared_data: D,
    output: PathAndName,
    render: impl FnOnce(D) -> Result<Pdf, E>,
    map_create_output_dir_error: impl Fn(std::io::Error) -> E,
    map_save_pdf_error: impl Fn(String) -> E,
) -> Result<AbstractNamedPdf<D>, E>
where
    D: Clone,
{
    let output_path = output.path().clone();
    create_folder_to_parent_of_path_if_needed(&output_path).map_err(map_create_output_dir_error)?;
    let pdf = render(prepared_data.clone())?;
    save_pdf(pdf.clone(), &output_path).map_err(map_save_pdf_error)?;
    Ok(AbstractNamedPdf::builder()
        .pdf(pdf)
        .saved_at(output_path)
        .name(output.name().clone())
        .prepared_data(prepared_data)
        .build())
}

/// Full data-load -> prepare -> resolve-output -> render -> save pipeline for PDFs.
pub fn create_pdf_document<I, Data, PreparedData, E>(
    input: I,
    load_data: impl FnOnce() -> Result<Data, E>,
    prepare_data: impl FnOnce(Data, I) -> Result<PreparedData, E>,
    resolve_output: impl FnOnce(&PreparedData) -> Result<PathAndName, E>,
    render: impl FnOnce(PreparedData) -> Result<Pdf, E>,
    map_create_output_dir_error: impl Fn(std::io::Error) -> E,
    map_save_pdf_error: impl Fn(String) -> E,
) -> Result<AbstractNamedPdf<PreparedData>, E>
where
    PreparedData: Clone,
{
    let data = load_data()?;
    let prepared_data = prepare_data(data, input)?;
    let output = resolve_output(&prepared_data)?;
    render_and_save_named_pdf(
        prepared_data,
        output,
        render,
        map_create_output_dir_error,
        map_save_pdf_error,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Prepared {
        bytes: Vec<u8>,
    }

    #[test]
    fn pipeline_builds_pdf_and_persists_to_disk() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let out = OutputPath::AbsolutePath(tmp.path().to_path_buf());

        let named_pdf = create_pdf_document(
            (),
            || {
                Ok::<Prepared, String>(Prepared {
                    bytes: vec![1, 2, 3],
                })
            },
            |data, _| Ok::<Prepared, String>(data),
            |prepared| {
                let _ = prepared;
                resolve_output_path_and_name(&out, "ignored")
            },
            |prepared| Ok::<Pdf, String>(Pdf::from(prepared.bytes)),
            |e| format!("{e:?}"),
            |e| e,
        )
        .unwrap();

        assert_eq!(named_pdf.pdf().as_ref(), &vec![1, 2, 3]);
        assert!(named_pdf.saved_at().exists());
    }
}
