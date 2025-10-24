use aes_gcm::Key;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::prelude::*;

/// A 32 bytes symmetric encryption key for AES GCM 256 encryption.
/// This key is used to encrypt and decrypt data securely.
#[derive(
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    From,
    Serialize,
    Deref,
    Deserialize,
    Hash,
    Zeroize,
    ZeroizeOnDrop,
)]
#[display("{}", hex::encode(self.0))]
#[serde(transparent)]
pub struct EncryptionKey(pub [u8; 32]);

impl From<EncryptionKey> for Key<aes_gcm::Aes256Gcm> {
    fn from(value: EncryptionKey) -> Self {
        Self::from(value.0)
    }
}
