use crate::prelude::*;

pub type AesNonce = [u8; 12];

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct AesGcmSealedBox {
    /// Nonce is 12 bytes
    pub(super) nonce: AesNonce,

    /// Auth tag and encrypted payload
    pub(super) cipher_text: Vec<u8>,
}

impl AesGcmSealedBox {
    pub const AUTH_TAG_LEN: usize = 16;
    pub const NONCE_LEN: usize = 12;
    pub const LOWER_BOUND_LEN: usize = Self::AUTH_TAG_LEN + Self::NONCE_LEN + 1; // at least 1 byte cipher. VERY much LOWER bound

    pub(super) fn combined(self) -> Vec<u8> {
        let mut combined = Vec::<u8>::new();
        let mut nonce = self.nonce.to_vec();
        let mut cipher_text = self.cipher_text;
        combined.append(&mut nonce);
        combined.append(&mut cipher_text);
        assert!(combined.len() >= Self::LOWER_BOUND_LEN);
        combined
    }
}

impl TryFrom<&[u8]> for AesGcmSealedBox {
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < Self::LOWER_BOUND_LEN {
            return Err(Error::InvalidAESBytesTooShort {
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
