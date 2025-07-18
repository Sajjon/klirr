use crate::{logic::Salt, prelude::*};

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[serde_as]
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Display,
    derive_more::Debug,
    Zeroize,
    ZeroizeOnDrop,
)]
#[display("{}", hex::encode(&self.0))]
#[debug("{}", hex::encode(&self.0))]
#[serde(transparent)]
pub struct EncryptedAppPassword(#[serde_as(as = "serde_with::hex::Hex")] Vec<u8>);

impl HasSample for SecretString {
    fn sample() -> Self {
        Self::from("encryption password")
    }

    fn sample_other() -> Self {
        Self::from("another encryption password")
    }
}

impl HasSample for EncryptedAppPassword {
    fn sample() -> Self {
        // SecretString::from("super secret"),
        // SecretString::sample(),
        // Salt::sample()
        Self(
            hex::decode(
                "3219e571fbb18265b1fb3f36a75c8e7ef4feef52892a5be25d0b9a92154c5de6456cdfe66aa70070",
            )
            .unwrap(),
        )
    }

    fn sample_other() -> Self {
        // Self::new_by_deriving_and_encrypting(
        //     SecretString::from("another super secret"),
        //     SecretString::sample_other(),
        //     &Salt::sample_other(),
        // )
        Self(
            hex::decode(
                "5b4d6fb8f3bc35af4168b6a0e593e69bedc75a9a062b77a36d6d01cbec06faaaaa3b89fbfd4b5b077c0ae0775de5ac1d",
            )
            .unwrap(),
        )
    }
}

impl EncryptedAppPassword {
    pub fn new_by_deriving_and_encrypting(
        app_password: SecretString,
        encryption_password: SecretString,
        salt: &Salt,
    ) -> Self {
        let encryption_key = PbHkdfSha256::derive_key_from(encryption_password, salt);
        Self::new_by_encrypting(app_password, encryption_key)
    }

    pub fn new_by_encrypting(app_password: SecretString, encryption_key: EncryptionKey) -> Self {
        let sealed_box = AesGcm256::seal(app_password.expose_secret().as_bytes(), encryption_key);
        let combined = sealed_box.combined();
        Self(combined)
    }

    pub fn derive_and_decrypt(
        &self,
        encryption_password: SecretString,
        salt: &Salt,
    ) -> Result<SecretString> {
        let encryption_key = PbHkdfSha256::derive_key_from(encryption_password, salt);
        self.decrypt(encryption_key)
    }

    pub fn decrypt(&self, encryption_key: EncryptionKey) -> Result<SecretString> {
        let sealed_box = AesGcmSealedBox::try_from(self.0.as_slice())?;
        let decrypted = AesGcm256::open(sealed_box, encryption_key)?;
        String::from_utf8(decrypted)
            .map_err(|_| Error::InvalidUtf8)
            .map(SecretString::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = EncryptedAppPassword;

    #[test]
    fn equality_secret_str() {
        assert_ne!(
            SecretString::sample().expose_secret(),
            SecretString::sample_other().expose_secret()
        );
    }

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
    fn test_encrypted_app_password() {
        let app_password = SecretString::from("my_secret_app_password");
        let encryption_pwd = SecretString::from("open sesame");
        let salt = Salt::sample();
        let encrypted = Sut::new_by_deriving_and_encrypting(
            app_password.clone(),
            encryption_pwd.clone(),
            &salt,
        );
        let decrypted = encrypted.derive_and_decrypt(encryption_pwd, &salt).unwrap();

        assert_eq!(decrypted.expose_secret(), app_password.expose_secret());
    }

    #[test]
    fn test_decrypt_invalid_utf8() {
        // Create encryption key
        let encryption_pwd = SecretString::from("key");
        let salt = Salt::generate();
        let encryption_key = PbHkdfSha256::derive_key_from(encryption_pwd, &salt);

        // Encrypt some invalid UTF-8 bytes directly
        let invalid_utf8_bytes = vec![0xFF, 0xFE, 0xFD]; // Invalid UTF-8 sequence
        let sealed_box = AesGcm256::seal(&invalid_utf8_bytes, encryption_key);
        let malformed_encrypted = EncryptedAppPassword(sealed_box.combined());

        // Create a new key for decryption (since the first one was moved)
        let decryption_key = PbHkdfSha256::derive_key_from(SecretString::from("key"), &salt);

        // Try to decrypt - should fail with InvalidUtf8
        let result = malformed_encrypted.decrypt(decryption_key);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidUtf8));
    }
}
