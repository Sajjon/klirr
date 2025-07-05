use crate::prelude::*;

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

impl HasSample for EncryptedAppPassword {
    fn sample() -> Self {
        Self::new_by_deriving_and_encrypting(
            SecretString::from("super secret"),
            SecretString::from("encryption password"),
        )
    }
}

impl EncryptedAppPassword {
    pub fn new_by_deriving_and_encrypting(
        app_password: SecretString,
        encryption_password: SecretString,
    ) -> Self {
        let encryption_key = PbHkdfSha256::derive_key_from(encryption_password);
        Self::new_by_encrypting(app_password, encryption_key)
    }

    pub fn new_by_encrypting(app_password: SecretString, encryption_key: EncryptionKey) -> Self {
        let sealed_box = AesGcm256::seal(app_password.expose_secret().as_bytes(), encryption_key);
        let combined = sealed_box.combined();
        Self(combined)
    }

    pub fn decrypt(&self, encryption_key: EncryptionKey) -> Result<String> {
        let sealed_box = AesGcmSealedBox::try_from(self.0.as_slice())?;
        let decrypted = AesGcm256::open(sealed_box, encryption_key)?;
        String::from_utf8(decrypted).map_err(|_| Error::InvalidUtf8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypted_app_password() {
        let app_password = SecretString::from("my_secret_app_password");
        let encryption_pwd = SecretString::from("open sesame");
        let encrypted = EncryptedAppPassword::new_by_deriving_and_encrypting(
            app_password.clone(),
            encryption_pwd.clone(),
        );
        let decrypted = encrypted
            .decrypt(PbHkdfSha256::derive_key_from(encryption_pwd))
            .unwrap();

        assert_eq!(decrypted, app_password.expose_secret());
    }
}
