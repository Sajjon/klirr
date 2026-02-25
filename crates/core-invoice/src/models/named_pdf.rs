use klirr_foundation::AbstractNamedPdf;

use crate::PreparedData;

pub type NamedPdf = AbstractNamedPdf<PreparedData>;

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
