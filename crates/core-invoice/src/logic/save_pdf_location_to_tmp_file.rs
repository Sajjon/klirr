use crate::prelude::*;

/// Tries to get the path to a temporary file where the path to the PDF will be saved.
/// This is determined by the environment variable `TMP_FILE_FOR_PATH_TO_PDF`.
///
/// Typically this is set by the makefile to a tmp file, used so that the
/// make file can open the PDF automatically after it has been created.
fn get_tmp_file_for_path_to_pdf() -> Option<PathBuf> {
    let path_to_tmp_file_where_we_write_dir_of_pdf =
        std::env::var("TMP_FILE_FOR_PATH_TO_PDF").ok()?;
    Some(PathBuf::from(path_to_tmp_file_where_we_write_dir_of_pdf))
}

/// Saves the path to the PDF file to a temporary file, if the environment variable
/// `TMP_FILE_FOR_PATH_TO_PDF` is set.
pub fn save_pdf_location_to_tmp_file(pdf_location: PathBuf) -> Result<()> {
    save_pdf_location_to_tmp_file_target(pdf_location, get_tmp_file_for_path_to_pdf())
}

fn save_pdf_location_to_tmp_file_target(
    pdf_location: PathBuf,
    target: Option<PathBuf>,
) -> Result<()> {
    let Some(target) = target else {
        return Ok(());
    };
    let path = target.display().to_string();
    trace!("Saving path to PDF to temp file '{}'", path);
    if let Err(e) = std::fs::write(&target, pdf_location.to_string_lossy().as_bytes()) {
        warn!("⚠️ Write to {path}: {e} (scripts won't find PDF.)",);
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use test_log::test;

    #[test]
    fn test_save_pdf_location_to_tmp_file() {
        {
            // Without env set
            let pdf_location = PathBuf::from("test.pdf");
            let result =
                save_pdf_location_to_tmp_file_target(pdf_location.clone(), Some(PathBuf::new()));
            assert!(result.is_ok());
        }
        {
            // With env set
            let tmp_file = NamedTempFile::new().unwrap();
            let tmp_file_path = tmp_file.path().to_path_buf();
            unsafe {
                std::env::set_var(
                    "TMP_FILE_FOR_PATH_TO_PDF",
                    tmp_file_path.display().to_string(),
                );
            }
            let pdf_location = PathBuf::from("test.pdf");
            let result = save_pdf_location_to_tmp_file(pdf_location.clone());
            assert!(result.is_ok());
            let content = std::fs::read_to_string(tmp_file_path).unwrap();
            assert_eq!(content, pdf_location.to_string_lossy());
        }
    }
}
