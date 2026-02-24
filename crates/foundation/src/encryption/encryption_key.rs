use aes_gcm::Key;
use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A 32-byte symmetric key used with AES-GCM-256.
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
