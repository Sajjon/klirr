use bon::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

use super::{CryptoError, Result};

pub type AesNonce = [u8; 12];

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Builder, Getters)]
pub struct AesGcmSealedBox {
    /// Nonce is 12 bytes
    #[getset(get = "pub")]
    nonce: AesNonce,

    /// Auth tag and encrypted payload
    #[getset(get = "pub")]
    cipher_text: Vec<u8>,
}

impl AesGcmSealedBox {
    const AUTH_TAG_LEN: usize = 16;
    const NONCE_LEN: usize = 12;
    const LOWER_BOUND_LEN: usize = Self::AUTH_TAG_LEN + Self::NONCE_LEN + 1;

    pub fn combined(self) -> Vec<u8> {
        let mut combined = Vec::with_capacity(self.nonce.len() + self.cipher_text.len());
        let mut nonce = self.nonce.to_vec();
        let mut cipher_text = self.cipher_text;
        combined.append(&mut nonce);
        combined.append(&mut cipher_text);
        assert!(combined.len() >= Self::LOWER_BOUND_LEN);
        combined
    }
}

impl TryFrom<&[u8]> for AesGcmSealedBox {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < Self::LOWER_BOUND_LEN {
            return Err(CryptoError::InvalidAesBytesTooShort {
                expected_at_least: Self::LOWER_BOUND_LEN,
                found: bytes.len(),
            });
        }

        let nonce_bytes = &bytes[..Self::NONCE_LEN];
        let nonce =
            AesNonce::try_from(nonce_bytes).expect("Should have exactly 12 bytes for nonce");
        let cipher_text = &bytes[Self::NONCE_LEN..];
        Ok(Self {
            nonce,
            cipher_text: cipher_text.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_few_bytes() {
        let bytes = [0; AesGcmSealedBox::LOWER_BOUND_LEN - 1];
        let result = AesGcmSealedBox::try_from(&bytes[..]);
        assert!(result.is_err());
    }
}
