use crate::prelude::*;

/// The outcome of generating a PDF with the produced PDF, its name, save location,
/// and the prepared data used to generate it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder, Getters)]
pub struct NamedPdf {
    /// The prepared data used to generate the PDF, e.g. invoice data.
    #[getset(get = "pub")]
    prepared_data: PreparedData,

    /// The generated PDF document.
    #[getset(get = "pub")]
    pdf: Pdf,

    /// The path where the PDF is saved, e.g. "/tmp/invoice_123.pdf"
    #[getset(get = "pub")]
    saved_at: PathBuf,

    /// The name of the PDF file, e.g. "invoice_123.pdf"
    #[getset(get = "pub")]
    name: String,
}

impl HasSample for NamedPdf {
    fn sample() -> Self {
        Self::builder()
            .prepared_data(PreparedData::sample())
            .pdf(Pdf::sample()) // Sample PDF data
            .saved_at(PathBuf::from("/tmp/sample_invoice.pdf"))
            .name("sample_invoice.pdf".to_string())
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .prepared_data(PreparedData::sample_other())
            .pdf(Pdf::sample_other()) // Another sample PDF data
            .saved_at(PathBuf::from("/tmp/another_invoice.pdf"))
            .name("another_invoice.pdf".to_string())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = NamedPdf;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
