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
    use std::ffi::OsString;
    use tempfile::NamedTempFile;
    use tempfile::tempdir;

    fn restore_env_var(name: &str, previous: Option<OsString>) {
        match previous {
            Some(value) => unsafe { std::env::set_var(name, value) },
            None => unsafe { std::env::remove_var(name) },
        }
    }

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

    #[test]
    fn save_pdf_location_to_tmp_file_target_logs_on_write_error() {
        let dir = tempdir().unwrap();
        save_pdf_location_to_tmp_file_target(
            PathBuf::from("test.pdf"),
            Some(dir.path().to_path_buf()),
        );
    }

    #[test]
    fn data_dir_create_if_true_creates_directory() {
        let tmp = tempdir().unwrap();
        let previous_xdg_data_home = std::env::var_os("XDG_DATA_HOME");
        let previous_home = std::env::var_os("HOME");

        unsafe {
            std::env::set_var("XDG_DATA_HOME", tmp.path());
            std::env::set_var("HOME", tmp.path());
        }

        let dir = data_dir_create_if(true);
        assert!(dir.ends_with("klirr/data"));
        assert!(dir.is_dir());

        restore_env_var("XDG_DATA_HOME", previous_xdg_data_home);
        restore_env_var("HOME", previous_home);
    }

    #[test]
    fn data_dir_matches_non_creating_variant() {
        assert_eq!(data_dir(), data_dir_create_if(false));
    }
}
