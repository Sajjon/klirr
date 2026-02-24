use thiserror::Error;

/// Result alias for render-pdf operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that can occur while rendering Typst documents into PDF bytes.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to build an in-memory Typst source file.
    #[error("Failed to load typst source, because: {underlying}")]
    LoadSource {
        /// Underlying load/parse error message.
        underlying: String,
    },
    /// Typst compilation failed.
    #[error("Failed to compile Typst source, because: {underlying}")]
    BuildPdf {
        /// Underlying compile error message.
        underlying: String,
    },
    /// PDF export from a compiled Typst document failed.
    #[error("Failed to export document to PDF, because: {underlying}")]
    ExportDocumentToPdf {
        /// Underlying PDF-export error message.
        underlying: String,
    },
    /// The requested font family could not be loaded.
    #[error("Failed to load font '{family_name}'")]
    FailedToLoadFont {
        /// Font family name that failed to load.
        family_name: String,
    },
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

#[cfg(test)]
mod tests {
    use super::Error;
    use std::fmt;

    struct DebugPassthrough(&'static str);
    impl fmt::Debug for DebugPassthrough {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn load_source_keeps_underlying_message() {
        let err = Error::load_source("inline source parse failed");
        assert!(matches!(
            err,
            Error::LoadSource { underlying } if underlying == "inline source parse failed"
        ));
    }

    #[test]
    fn build_pdf_keeps_underlying_message() {
        let err = Error::build_pdf(DebugPassthrough("compile failed"));
        assert!(matches!(
            err,
            Error::BuildPdf { underlying } if underlying == "compile failed"
        ));
    }

    #[test]
    fn export_document_to_pdf_keeps_underlying_message() {
        let err = Error::export_document_to_pdf(DebugPassthrough("pdf backend failed"));
        assert!(matches!(
            err,
            Error::ExportDocumentToPdf { underlying } if underlying == "pdf backend failed"
        ));
    }
}
