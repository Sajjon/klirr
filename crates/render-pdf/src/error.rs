use thiserror::Error;

/// Result alias for render-pdf operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that can occur while rendering Typst documents into PDF bytes.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to build an in-memory Typst source file.
    #[error("Failed to load typst source, because: {underlying}")]
    LoadSource { underlying: String },
    /// Typst compilation failed.
    #[error("Failed to compile Typst source, because: {underlying}")]
    BuildPdf { underlying: String },
    /// PDF export from a compiled Typst document failed.
    #[error("Failed to export document to PDF, because: {underlying}")]
    ExportDocumentToPdf { underlying: String },
    /// The requested font family could not be loaded.
    #[error("Failed to load font '{family_name}'")]
    FailedToLoadFont { family_name: String },
}

impl Error {
    /// Creates a [`Error::LoadSource`] from a displayable source error.
    pub fn load_source(underlying: impl std::fmt::Display) -> Self {
        Self::LoadSource {
            underlying: underlying.to_string(),
        }
    }

    /// Creates a [`Error::BuildPdf`] from a debug-formatted source error.
    pub fn build_pdf(underlying: impl std::fmt::Debug) -> Self {
        Self::BuildPdf {
            underlying: format!("{underlying:?}"),
        }
    }

    /// Creates a [`Error::ExportDocumentToPdf`] from a debug-formatted source error.
    pub fn export_document_to_pdf(underlying: impl std::fmt::Debug) -> Self {
        Self::ExportDocumentToPdf {
            underlying: format!("{underlying:?}"),
        }
    }
}
