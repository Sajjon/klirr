use aes_gcm::{
    Key,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

use super::{AesGcmSealedBox, AesNonce, CryptoError, EncryptionKey, Result};

/// AES-GCM-256 encryption helper.
#[derive(Clone, Default, PartialEq, Eq, Hash, derive_more::Display, derive_more::Debug)]
pub struct AesGcm256;

impl AesGcm256 {
    /// Seals the plaintext using AES GCM 256 encryption with the provided encryption key.
    /// Returns an `AesGcmSealedBox` containing the encrypted data and nonce.
    ///
    /// # Arguments
    /// * `plaintext` - The data to encrypt.
    /// * `encryption_key` - The key used for encryption, must be 32 bytes long.
    ///
    /// # Returns
    /// An `AesGcmSealedBox` containing the encrypted data and nonce.
    fn _seal(
        plaintext: impl AsRef<[u8]>,
        encryption_key: Key<aes_gcm::Aes256Gcm>,
    ) -> AesGcmSealedBox {
        let cipher = aes_gcm::Aes256Gcm::new(&encryption_key);
        let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut OsRng);
        let cipher_text = cipher
            .encrypt(&nonce, plaintext.as_ref())
            .expect("AES encrypt never fails for valid nonce.");
        let nonce = AesNonce::try_from(nonce.as_slice()).unwrap();

        AesGcmSealedBox::builder()
            .nonce(nonce)
            .cipher_text(cipher_text)
            .build()
    }

    /// Opens the sealed box by decrypting the cipher text using the provided decryption key.
    /// Returns the decrypted plaintext.
    /// # Arguments
    /// * `sealed_box` - The sealed box containing the encrypted data and nonce.
    /// * `decryption_key` - The key used for decryption, must be 32 bytes long.
    fn _open(
        sealed_box: AesGcmSealedBox,
        decryption_key: Key<aes_gcm::Aes256Gcm>,
    ) -> Result<Vec<u8>> {
        let cipher = aes_gcm::Aes256Gcm::new(&decryption_key);
        let cipher_text = sealed_box.cipher_text();
        cipher
            .decrypt(sealed_box.nonce().into(), cipher_text.as_ref())
            .map_err(CryptoError::aes_decryption_failed)
    }

    /// Seals the plaintext using AES GCM 256 encryption with the provided encryption key.
    /// Returns an `AesGcmSealedBox` containing the encrypted data and nonce
    /// # Arguments
    /// * `plaintext` - The data to encrypt.
    /// * `encryption_key` - The key used for encryption, must be 32 bytes long.
    /// # Returns
    /// An `AesGcmSealedBox` containing the encrypted data and nonce.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_foundation;
    /// use klirr_foundation::*;
    /// let secret = "super secret data";
    /// let encryption_key = EncryptionKey([0xabu8; 32]);
    /// // Seal the plaintext
    /// let encrypted = AesGcm256::seal(secret.as_bytes(), encryption_key.clone());
    /// let decrypted = AesGcm256::open(encrypted, encryption_key).unwrap();
    /// assert_eq!(decrypted, secret.as_bytes());
    /// ```
    pub fn seal(plaintext: impl AsRef<[u8]>, encryption_key: EncryptionKey) -> AesGcmSealedBox {
        Self::_seal(plaintext, encryption_key.into())
    }

    /// Opens the sealed box by decrypting the cipher text using the provided decryption key.
    /// Returns the decrypted plaintext.
    /// # Arguments
    /// * `sealed_box` - The sealed box containing the encrypted data and nonce.
    /// * `decryption_key` - The key used for decryption, must be 32 bytes long.
    /// # Returns
    /// The decrypted plaintext as a `Vec<u8>`.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_foundation;
    /// use klirr_foundation::*;
    /// let secret = "super secret data";
    /// let encryption_key = EncryptionKey([0xabu8; 32]);
    /// // Seal the plaintext
    /// let encrypted = AesGcm256::seal(secret.as_bytes(), encryption_key.clone());
    /// let decrypted = AesGcm256::open(encrypted, encryption_key).unwrap();
    /// assert_eq!(decrypted, secret.as_bytes());
    /// ```
    pub fn open(sealed_box: AesGcmSealedBox, decryption_key: EncryptionKey) -> Result<Vec<u8>> {
        Self::_open(sealed_box, decryption_key.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode as hex_decode;

    fn sample_encryption_key() -> EncryptionKey {
        EncryptionKey([0xabu8; 32])
    }

    #[test]
    fn test_decrypt() {
        let combined = hex_decode("ae8e7654ded1c276d5c428b10bef17f2a3b885e156a853e781fabe219fa19e5780c1a57a51a58c7384e69545da6a83bf4f").unwrap();
        let sealed_box = AesGcmSealedBox::try_from(combined.as_slice()).unwrap();
        let decrypted = AesGcm256::open(sealed_box, sample_encryption_key()).unwrap();
        let decrypted_str = String::from_utf8(decrypted).unwrap();
        assert_eq!(decrypted_str, "yay decryption worked");
    }

    #[test]
    fn test_roundtrip() {
        let plaintext = "so super secret".to_owned();
        let encryption_key = sample_encryption_key();
        let sealed = AesGcm256::seal(plaintext.clone(), encryption_key.clone());
        let decrypted = AesGcm256::open(sealed.clone(), encryption_key).unwrap();
        let decrypted_str = String::from_utf8(decrypted).unwrap();
        assert_eq!(plaintext, decrypted_str);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            AesGcm256::open(
                AesGcmSealedBox::builder()
                    .nonce([0xabu8; 12])
                    .cipher_text(hex_decode("deadbeef").unwrap())
                    .build(),
                sample_encryption_key()
            ),
            Err(CryptoError::AesDecryptionFailed)
        )
    }
}
