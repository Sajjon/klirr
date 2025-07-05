use crate::prelude::*;

pub type AesNonce = [u8; 12];

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, TypedBuilder, Getters)]
pub struct AesGcmSealedBox {
    /// Nonce is 12 bytes
    #[builder(setter(into))]
    #[getset(get = "pub")]
    nonce: AesNonce,

    /// Auth tag and encrypted payload
    #[builder(setter(into))]
    #[getset(get = "pub")]
    cipher_text: Vec<u8>,
}

impl AesGcmSealedBox {
    const AUTH_TAG_LEN: usize = 16;
    const NONCE_LEN: usize = 12;
    const LOWER_BOUND_LEN: usize = Self::AUTH_TAG_LEN + Self::NONCE_LEN + 1; // at least 1 byte cipher. VERY much LOWER bound

    pub(super) fn combined(self) -> Vec<u8> {
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
