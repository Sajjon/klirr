use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to load typst source, because: {underlying}")]
    LoadSource { underlying: String },
    #[error("Failed to compile Typst source, because: {underlying}")]
    BuildPdf { underlying: String },
    #[error("Failed to export document to PDF, because: {underlying}")]
    ExportDocumentToPdf { underlying: String },
    #[error("Failed to load font '{family_name}'")]
    FailedToLoadFont { family_name: String },
}
