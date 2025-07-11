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
