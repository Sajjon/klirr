/// Result alias for crypto operations.
pub type Result<T, E = CryptoError> = std::result::Result<T, E>;

/// Errors for feature-gated crypto utilities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CryptoError {
    InvalidUtf8,
    AesDecryptionFailed,
    InvalidAesBytesTooShort {
        expected_at_least: usize,
        found: usize,
    },
}

impl CryptoError {
    pub fn aes_decryption_failed(_error: impl std::fmt::Debug) -> Self {
        Self::AesDecryptionFailed
    }
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUtf8 => write!(f, "Failed to parse string into a valid UTF-8 string"),
            Self::AesDecryptionFailed => write!(f, "Failed to decrypt data with AES"),
            Self::InvalidAesBytesTooShort {
                expected_at_least,
                found,
            } => write!(
                f,
                "Invalid AES bytes, expected at least {} bytes, but found {} bytes",
                expected_at_least, found
            ),
        }
    }
}

impl std::error::Error for CryptoError {}
