use derive_more::{AsRef, Deref, From};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A cryptographically secure random salt used for key derivation.
#[serde_as]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    From,
    Deref,
    AsRef,
    Serialize,
    Deserialize,
    Zeroize,
    ZeroizeOnDrop,
)]
#[serde(transparent)]
pub struct Salt(#[serde_as(as = "serde_with::hex::Hex")] [u8; 16]);

impl Salt {
    /// Uses CSPRNG to generate a random salt.
    pub fn generate() -> Self {
        use rand::RngCore;
        let mut salt = [0u8; 16];
        rand::rng().fill_bytes(&mut salt);
        Self(salt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salt_generate() {
        let salt1 = Salt::generate();
        let salt2 = Salt::generate();
        assert_ne!(salt1, salt2);
        assert_ne!(*salt1, [0u8; 16]);
        assert_ne!(*salt2, [0u8; 16]);
    }
}
