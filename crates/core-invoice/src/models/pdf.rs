use crate::prelude::*;

pub use klirr_core_pdf::Pdf;

impl HasSample for Pdf {
    fn sample() -> Self {
        Pdf(vec![0xde, 0xad, 0xbe, 0xef])
    }

    fn sample_other() -> Self {
        Pdf(vec![0xca, 0xfe, 0xba, 0xbe])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = Pdf;

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
