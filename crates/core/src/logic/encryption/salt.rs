use serde_with::serde_as;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::prelude::*;

/// A cryptographically secure random salt used for key derivation.
/// It is used to ensure that the derived keys are unique even if the
/// input key material is the same.
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
    /// Uses CSPRNG (safe) to generate a salt.
    pub fn generate() -> Self {
        use rand::RngCore;
        let mut salt = [0u8; 16];
        rand::rng().fill_bytes(&mut salt);
        Self(salt)
    }
}

impl HasSample for Salt {
    fn sample() -> Self {
        Self([0xab; 16])
    }

    fn sample_other() -> Self {
        Self([0xcd; 16])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = Salt;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn test_salt_generate() {
        let salt1 = Salt::generate();
        let salt2 = Salt::generate();

        // Generated salts should be different
        assert_ne!(salt1, salt2);

        // Salt should not be all zeros
        assert_ne!(*salt1, [0u8; 16]);
        assert_ne!(*salt2, [0u8; 16]);
    }
}
