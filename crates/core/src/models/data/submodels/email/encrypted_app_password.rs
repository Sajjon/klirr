use crate::prelude::*;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(
    Clone, PartialEq, Eq, Hash, Serialize, Deserialize, derive_more::Display, derive_more::Debug,
)]
#[display("{}", hex::encode(&self.0))]
#[debug("{}", hex::encode(&self.0))]
#[serde(transparent)]
pub struct EncryptedAppPassword(#[serde_as(as = "serde_with::hex::Hex")] Vec<u8>);

impl EncryptedAppPassword {
    pub fn new_by_deriving_and_encrypting(
        app_password: String,
        encryption_password: String,
    ) -> Self {
        let encryption_key = PbHkdfSha256::derive_key(encryption_password);
        Self::new_by_encrypting(app_password, encryption_key)
    }

    pub fn new_by_encrypting(app_password: String, encryption_key: EncryptionKey) -> Self {
        let sealed_box = AesGcm256::seal(app_password.as_bytes(), encryption_key);
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
        let app_password = "my_secret_app_password".to_string();
        let encryption_pwd = "open sesame".to_string();
        let encrypted = EncryptedAppPassword::new_by_deriving_and_encrypting(
            app_password.clone(),
            encryption_pwd.clone(),
        );
        let decrypted = encrypted
            .decrypt(PbHkdfSha256::derive_key(encryption_pwd))
            .unwrap();

        assert_eq!(decrypted, app_password);
    }
}
