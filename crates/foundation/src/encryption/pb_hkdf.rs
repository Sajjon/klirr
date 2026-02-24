use hkdf::Hkdf;
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;

use super::{EncryptionKey, Salt};

/// A small HKDF-SHA256 key-derivation helper.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct PbHkdfSha256;

impl PbHkdfSha256 {
    const INFO: &'static [u8] = b"klirr email encryption";

    fn derive_key_with_ikm_salt_info(
        ikm: impl AsRef<[u8]>,
        salt: Option<&Salt>,
        info: Option<&[u8]>,
    ) -> [u8; 32] {
        let mut okm = [0u8; 32];
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
        let salt = Salt::from([0u8; 16]);
        let derived = PbHkdfSha256::derive_key(ikm, &salt);
        assert_eq!(
            hex_encode(*derived),
            "ce81a9fdee0e4db76e31d9b49ad2d5b09b45647bc56682d62110fbf32c2903b1"
        );
    }
}
