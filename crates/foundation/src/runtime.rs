use std::path::PathBuf;

use log::{trace, warn};

use crate::create_folder_if_needed;

pub const BINARY_NAME: &str = "klirr";
pub const TMP_FILE_FOR_PATH_TO_PDF_ENV: &str = "TMP_FILE_FOR_PATH_TO_PDF";

/// Returns the path to the data directory, which is typically located at
/// ```text
/// macOS: `~/Library/Application Support/klirr/data`
/// Linux: `~/.local/share/klirr/data`
/// Windows: `C:\Users\Alice\AppData\Local\klirr\data`
/// ```
///
/// Creates if `create_if_not_exists` is true and if needed.
///
/// For more information
/// see [dirs_next][ref]
///
/// [ref]: https://docs.rs/dirs-next/latest/dirs_next/fn.data_local_dir.html
pub fn data_dir_create_if(create_if_not_exists: bool) -> PathBuf {
    let dir = dirs_next::data_local_dir()
        .expect("Should have a data directory")
        .join(BINARY_NAME)
        .join("data");
    if create_if_not_exists {
        create_folder_if_needed(&dir)
            .expect("Should be able to create directory at data_dir()/klirr/data");
    }
    dir
}

/// Returns the app data directory without creating it.
pub fn data_dir() -> PathBuf {
    data_dir_create_if(false)
}

fn get_tmp_file_for_path_to_pdf() -> Option<PathBuf> {
    let path_to_tmp_file_where_we_write_dir_of_pdf =
        std::env::var(TMP_FILE_FOR_PATH_TO_PDF_ENV).ok()?;
    Some(PathBuf::from(path_to_tmp_file_where_we_write_dir_of_pdf))
}

/// Saves the PDF path to the file provided by `TMP_FILE_FOR_PATH_TO_PDF` if set.
///
/// Write failures are logged and intentionally ignored.
pub fn save_pdf_location_to_tmp_file(pdf_location: PathBuf) {
    save_pdf_location_to_tmp_file_target(pdf_location, get_tmp_file_for_path_to_pdf())
}

fn save_pdf_location_to_tmp_file_target(pdf_location: PathBuf, target: Option<PathBuf>) {
    let Some(target) = target else {
        return;
    };
    let path = target.display().to_string();
    trace!("Saving path to PDF to temp file '{}'", path);
    if let Err(e) = std::fs::write(&target, pdf_location.to_string_lossy().as_bytes()) {
        warn!("⚠️ Write to {path}: {e} (scripts won't find PDF.)",);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn save_pdf_location_to_tmp_file_writes_when_env_is_set() {
        let tmp_file = NamedTempFile::new().unwrap();
        let tmp_file_path = tmp_file.path().to_path_buf();
        unsafe {
            std::env::set_var(
                TMP_FILE_FOR_PATH_TO_PDF_ENV,
                tmp_file_path.display().to_string(),
            );
        }
        let pdf_location = PathBuf::from("test.pdf");
        save_pdf_location_to_tmp_file(pdf_location.clone());
        let content = std::fs::read_to_string(tmp_file_path).unwrap();
        assert_eq!(content, pdf_location.to_string_lossy());
    }

    #[test]
    fn save_pdf_location_to_tmp_file_target_no_target_is_noop() {
        save_pdf_location_to_tmp_file_target(PathBuf::from("test.pdf"), None);
    }
}
