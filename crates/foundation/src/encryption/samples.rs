use secrecy::SecretString;

use crate::HasSample;

use super::{EncryptedAppPassword, Salt};

impl HasSample for SecretString {
    fn sample() -> Self {
        Self::from("encryption password")
    }

    fn sample_other() -> Self {
        Self::from("another encryption password")
    }
}

impl HasSample for Salt {
    fn sample() -> Self {
        Self::from([0xab; 16])
    }

    fn sample_other() -> Self {
        Self::from([0xcd; 16])
    }
}

impl HasSample for EncryptedAppPassword {
    fn sample() -> Self {
        serde_json::from_value(serde_json::json!(
            "3219e571fbb18265b1fb3f36a75c8e7ef4feef52892a5be25d0b9a92154c5de6456cdfe66aa70070"
        ))
        .expect("valid encrypted app password hex")
    }

    fn sample_other() -> Self {
        serde_json::from_value(serde_json::json!(
            "5b4d6fb8f3bc35af4168b6a0e593e69bedc75a9a062b77a36d6d01cbec06faaaaa3b89fbfd4b5b077c0ae0775de5ac1d"
        ))
        .expect("valid encrypted app password hex")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::ExposeSecret;

    #[test]
    fn sample_secret_string_is_deterministic_and_distinct() {
        let sample = SecretString::sample();
        let sample_other = SecretString::sample_other();
        assert_eq!(sample.expose_secret(), "encryption password");
        assert_eq!(sample_other.expose_secret(), "another encryption password");
        assert_ne!(sample.expose_secret(), sample_other.expose_secret());
    }

    #[test]
    fn sample_salt_is_deterministic_and_distinct() {
        assert_eq!(Salt::sample(), Salt::from([0xab; 16]));
        assert_eq!(Salt::sample_other(), Salt::from([0xcd; 16]));
        assert_ne!(Salt::sample(), Salt::sample_other());
    }
}
