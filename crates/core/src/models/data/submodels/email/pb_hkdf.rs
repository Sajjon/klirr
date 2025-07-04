use crate::prelude::*;
use hkdf::Hkdf;
use sha2::Sha256;

/// A simple `HKDF` based scheme using UTF8 encoding of the password as input.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct PbHkdfSha256;

impl PbHkdfSha256 {
    const INFO: &'static [u8] = b"klirr email encryption";
    fn derive_key_with_ikm_salt_info<'a>(
        ikm: impl AsRef<[u8]>,
        salt: Option<&'a [u8]>,
        info: Option<&'a [u8]>,
    ) -> [u8; 32] {
        let mut okm = [0u8; 32]; // 32-byte buffer for the symmetric key

        let hkdf = Hkdf::<Sha256>::new(salt, ikm.as_ref());
        hkdf.expand(info.unwrap_or(&[]), &mut okm).unwrap();

        okm
    }

    pub fn derive_key(ikm: impl AsRef<[u8]>) -> EncryptionKey {
        Self::derive_key_with_ikm_salt_info(ikm, None, Some(Self::INFO)).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::encode as hex_encode;

    #[test]
    fn test_kdf() {
        let ikm = "open sesame";
        let derived = PbHkdfSha256::derive_key(ikm);
        assert_eq!(
            hex_encode(*derived),
            "ce81a9fdee0e4db76e31d9b49ad2d5b09b45647bc56682d62110fbf32c2903b1"
        );
    }
}
