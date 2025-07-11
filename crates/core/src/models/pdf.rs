use crate::prelude::*;

/// Bytes represents a PDF document in memory.
#[derive(Clone, Debug, From, AsRef, PartialEq, Eq, Hash)]
pub struct Pdf(pub Vec<u8>);

impl HasSample for Pdf {
    fn sample() -> Self {
        Self(vec![0xde, 0xad, 0xbe, 0xef]) // Sample PDF data
    }
    fn sample_other() -> Self {
        Self(vec![0xca, 0xfe, 0xba, 0xbe]) // Another sample PDF data
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder, Getters)]
pub struct NamedPdf {
    #[getset(get = "pub")]
    prepared_data: PreparedData,
    #[getset(get = "pub")]
    pdf: Pdf,
    #[getset(get = "pub")]
    saved_at: PathBuf,
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
