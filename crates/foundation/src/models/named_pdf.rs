use std::path::PathBuf;

use crate::HasSample;
use crate::Pdf;
use bon::Builder;
use getset::Getters;

/// The outcome of generating a PDF with the produced PDF, its name, save location,
/// and the prepared data used to generate it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder, Getters)]
pub struct AbstractNamedPdf<D> {
    /// The prepared data used to generate the PDF, e.g. invoice data.
    #[getset(get = "pub")]
    prepared_data: D,

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

impl<D: HasSample> HasSample for AbstractNamedPdf<D> {
    fn sample() -> Self {
        Self::builder()
            .prepared_data(D::sample())
            .pdf(Pdf::sample())
            .saved_at(PathBuf::from("/tmp/sample.pdf"))
            .name("sample.pdf".to_string())
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .prepared_data(D::sample_other())
            .pdf(Pdf::sample_other())
            .saved_at(PathBuf::from("/tmp/another_sample.pdf"))
            .name("another_sample.pdf".to_string())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    type Sut = AbstractNamedPdf<Day>;

    #[test]
    fn sample_populates_expected_fields() {
        let sample = Sut::sample();
        assert_eq!(*sample.prepared_data(), Day::sample());
        assert_eq!(*sample.pdf(), Pdf::sample());
        assert_eq!(sample.saved_at(), &PathBuf::from("/tmp/sample.pdf"));
        assert_eq!(sample.name(), "sample.pdf");
    }

    #[test]
    fn sample_other_populates_expected_fields() {
        let sample = Sut::sample_other();
        assert_eq!(*sample.prepared_data(), Day::sample_other());
        assert_eq!(*sample.pdf(), Pdf::sample_other());
        assert_eq!(sample.saved_at(), &PathBuf::from("/tmp/another_sample.pdf"));
        assert_eq!(sample.name(), "another_sample.pdf");
    }
}
