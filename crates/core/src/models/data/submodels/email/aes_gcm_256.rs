use crate::prelude::*;

use aes_gcm::{
    Key,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use zeroize::Zeroize;

/// AES GCM 256 encryption
#[derive(Clone, Default, PartialEq, Eq, Hash, derive_more::Display, derive_more::Debug)]
pub struct AesGcm256 {}

impl AesGcm256 {
    fn _seal(
        plaintext: impl AsRef<[u8]>,
        encryption_key: Key<aes_gcm::Aes256Gcm>,
    ) -> AesGcmSealedBox {
        let cipher = aes_gcm::Aes256Gcm::new(&encryption_key);
        let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut OsRng); // 12 bytes; unique per message
        let cipher_text = cipher
            .encrypt(&nonce, plaintext.as_ref())
            .expect("AES encrypt never fails for valid nonce.");
        let nonce = AesNonce::try_from(nonce.as_slice()).unwrap();

        AesGcmSealedBox { nonce, cipher_text }
    }

    fn _open(
        sealed_box: AesGcmSealedBox,
        decryption_key: Key<aes_gcm::Aes256Gcm>,
    ) -> Result<Vec<u8>> {
        let cipher = aes_gcm::Aes256Gcm::new(&decryption_key);
        let cipher_text = sealed_box.cipher_text;
        cipher
            .decrypt(sealed_box.nonce.as_ref().into(), cipher_text.as_ref())
            .map_err(|e| {
                error!("Failed to AES decrypt data - error: {:?}", e);
                Error::AESDecryptionFailed
            })
    }
}

impl AesGcm256 {
    pub fn seal(plaintext: impl AsRef<[u8]>, encryption_key: EncryptionKey) -> AesGcmSealedBox {
        Self::_seal(plaintext, encryption_key.into())
    }

    pub fn open(sealed_box: AesGcmSealedBox, decryption_key: EncryptionKey) -> Result<Vec<u8>> {
        Self::_open(sealed_box, decryption_key.into())
    }
}

#[derive(
    Zeroize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    From,
    Serialize,
    Deref,
    Deserialize,
    Hash,
)]
#[display("{}", hex::encode(self.0))]
#[serde(transparent)]
pub struct EncryptionKey(pub [u8; 32]);

impl From<EncryptionKey> for Key<aes_gcm::Aes256Gcm> {
    fn from(value: EncryptionKey) -> Self {
        Self::from(value.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use hex::decode as hex_decode;

    type Sut = AesGcm256;

    fn sample_encryption_key() -> EncryptionKey {
        EncryptionKey([0xabu8; 32])
    }

    #[test]
    fn test_decrypt() {
        let combined = hex_decode("ae8e7654ded1c276d5c428b10bef17f2a3b885e156a853e781fabe219fa19e5780c1a57a51a58c7384e69545da6a83bf4f").unwrap();
        let sealed_box = AesGcmSealedBox::try_from(combined.as_slice()).unwrap();
        let decrypted = Sut::open(sealed_box, sample_encryption_key()).unwrap();
        let decrypted_str = String::from_utf8(decrypted).unwrap();
        assert_eq!(decrypted_str, "yay decryption worked");
    }

    #[test]
    fn test_roundtrip() {
        let plaintext = "so super secret".to_owned();
        let encryption_key = sample_encryption_key();
        let sealed = Sut::seal(plaintext.clone(), encryption_key);
        let decrypted = Sut::open(sealed.clone(), encryption_key).unwrap();
        let decrypted_str = String::from_utf8(decrypted).unwrap();
        assert_eq!(plaintext, decrypted_str);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            Sut::open(
                AesGcmSealedBox {
                    nonce: [0xabu8; 12],
                    cipher_text: hex_decode("deadbeef").unwrap(),
                },
                sample_encryption_key()
            ),
            Err(Error::AESDecryptionFailed)
        )
    }
}
