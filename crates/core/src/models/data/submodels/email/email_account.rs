use crate::prelude::*;

/// A named sender and an email address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct EmailAccount {
    #[builder(setter(into))]
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
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_sample_values_hash() {
        assert_eq!(
            HashSet::from([EmailAccount::sample_alice(), EmailAccount::sample_bob()]).len(),
            2
        );
    }
}
