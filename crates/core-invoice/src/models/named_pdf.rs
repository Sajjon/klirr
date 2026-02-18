use klirr_foundation::{AbstractNamedPdf, Pdf};

use crate::{HasSample, PathBuf, PreparedData};

pub type NamedPdf = AbstractNamedPdf<PreparedData>;

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
    use crate::HasSample;

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
