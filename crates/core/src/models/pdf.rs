use crate::prelude::*;

/// Bytes represents a PDF document in memory.
#[derive(Clone, Debug, From, AsRef, PartialEq, Eq, Hash)]
pub struct Pdf(pub Vec<u8>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, TypedBuilder, Getters)]
pub struct NamedPdf {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    prepared_data: PreparedData,
    #[builder(setter(into))]
    #[getset(get = "pub")]
    pdf: Pdf,
    #[builder(setter(into))]
    #[getset(get = "pub")]
    saved_at: PathBuf,
    #[builder(setter(into))]
    #[getset(get = "pub")]
    name: String,
}

impl HasSample for NamedPdf {
    fn sample() -> Self {
        Self::builder()
            .prepared_data(PreparedData::sample())
            .pdf(Pdf(vec![0; 100])) // Sample PDF data
            .saved_at(PathBuf::from("/tmp/sample_invoice.pdf"))
            .name("sample_invoice.pdf".to_string())
            .build()
    }
}
