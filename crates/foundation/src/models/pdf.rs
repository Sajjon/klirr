use std::path::{Path, PathBuf};

use derive_more::{AsRef, From};
use log::info;

/// Bytes represents a PDF document in memory.
#[derive(Clone, Debug, From, AsRef, PartialEq, Eq, Hash)]
pub struct Pdf(pub Vec<u8>);

/// Formats an error using `Debug` for use in `save_pdf` string errors.
fn format_debug_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}, ")
}

/// Saves the PDF file `pdf` to the specified path `pdf_path`.
pub fn save_pdf(pdf: Pdf, pdf_path: impl AsRef<Path>) -> Result<PathBuf, String> {
    info!("Saving PDF to: '{}'", pdf_path.as_ref().display());
    // now save the PDF to a file
    let output_path = PathBuf::from(pdf_path.as_ref());
    std::fs::write(&output_path, pdf.as_ref()).map_err(format_debug_error)?;
    info!("✅ Saved PDF to: '{}'", pdf_path.as_ref().display());
    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(Pdf(vec![1, 2, 3, 4]), Pdf(vec![1, 2, 3, 4]));
    }

    #[test]
    fn inequality() {
        assert_ne!(Pdf(vec![1, 2, 3, 4]), Pdf(vec![4, 3, 2, 1]));
    }
}
