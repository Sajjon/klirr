use crate::prelude::*;

/// A named sender and an email address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder, Getters, Serialize, Deserialize)]
pub struct EmailAccount {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    email: EmailAddress,
}

impl EmailAccount {
    pub fn sample_alice() -> Self {
        Self::builder()
            .name("Alice Smith".to_string())
            .email(EmailAddress::sample_alice())
            .build()
    }

    pub fn sample_bob() -> Self {
        Self::builder()
            .name("Bob Johnson".to_string())
            .email(EmailAddress::sample_bob())
            .build()
    }
}

impl HasSample for EmailAccount {
    fn sample() -> Self {
        Self::sample_alice()
    }

    fn sample_other() -> Self {
        Self::sample_bob()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    type Sut = EmailAccount;

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
    fn test_sample_values_hash() {
        assert_eq!(
            HashSet::from([Sut::sample_alice(), Sut::sample_bob()]).len(),
            2
        );
    }
}
