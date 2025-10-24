use derive_more::{AsRef, From};

/// Bytes represents a PDF document in memory.
#[derive(Clone, Debug, From, AsRef, PartialEq, Eq, Hash)]
pub struct Pdf(pub Vec<u8>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(Pdf(vec![1, 2, 3, 4]), Pdf(vec![1, 2, 3, 4]));
    }

    #[test]
    fn inequality() {
        assert_ne!(Pdf(vec![1, 2, 3, 4]), Pdf(vec![4, 3, 2, 1]));
    }
}
