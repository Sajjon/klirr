use crate::prelude::*;
use hkdf::Hkdf;
use secrecy::{ExposeSecret, SecretString};
use serde_with::serde_as;
use sha2::Sha256;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A simple `HKDF` based scheme using UTF8 encoding of the password as input.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct PbHkdfSha256;

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
}

impl PbHkdfSha256 {
    const INFO: &'static [u8] = b"klirr email encryption";
    fn derive_key_with_ikm_salt_info(
        ikm: impl AsRef<[u8]>,
        salt: Option<&Salt>,
        info: Option<&[u8]>,
    ) -> [u8; 32] {
        let mut okm = [0u8; 32]; // 32-byte buffer for the symmetric key

        let hkdf = Hkdf::<Sha256>::new(salt.map(|s| s.as_ref() as &[u8]), ikm.as_ref());
        hkdf.expand(info.unwrap_or(&[]), &mut okm).unwrap();

        okm
    }

    pub fn derive_key(ikm: impl AsRef<[u8]>, salt: &Salt) -> EncryptionKey {
        Self::derive_key_with_ikm_salt_info(ikm, Some(salt), Some(Self::INFO)).into()
    }

    pub fn derive_key_from(password: SecretString, salt: &Salt) -> EncryptionKey {
        Self::derive_key(password.expose_secret(), salt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::encode as hex_encode;

    #[test]
    fn test_kdf() {
        let ikm = "open sesame";
        let salt = Salt([0u8; 16]); // Use a fixed salt for deterministic test
        let derived = PbHkdfSha256::derive_key(ikm, &salt);
        assert_eq!(
            hex_encode(*derived),
            "ce81a9fdee0e4db76e31d9b49ad2d5b09b45647bc56682d62110fbf32c2903b1"
        );
    }

    #[test]
    fn test_salt_generate() {
        let salt1 = Salt::generate();
        let salt2 = Salt::generate();

        // Generated salts should be different
        assert_ne!(salt1, salt2);

        // Salt should not be all zeros
        assert_ne!(salt1.0, [0u8; 16]);
        assert_ne!(salt2.0, [0u8; 16]);
    }
}
