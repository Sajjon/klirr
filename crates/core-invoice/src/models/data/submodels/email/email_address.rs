use crate::prelude::*;

/// A valid email address.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::FromStr,
    derive_more::Display,
    From,
    Deref,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[display("{}", _0)]
pub struct EmailAddress(lettre::Address);

impl HasSample for EmailAddress {
    fn sample() -> Self {
        Self::sample_alice()
    }

    fn sample_other() -> Self {
        Self::sample_bob()
    }
}

impl EmailAddress {
    pub fn sample_alice() -> Self {
        Self::from_str("alice@example.com").unwrap()
    }

    pub fn sample_bob() -> Self {
        Self::from_str("bob@example.com").unwrap()
    }

    pub fn sample_carol() -> Self {
        Self::from_str("carol@example.com").unwrap()
    }

    pub fn sample_dave() -> Self {
        Self::from_str("dave@example.com").unwrap()
    }

    pub fn sample_erin() -> Self {
        Self::from_str("erin@example.com").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    type Sut = EmailAddress;

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
    fn test_sample_is_alice() {
        assert_eq!(Sut::sample(), Sut::sample_alice());
    }

    #[test]
    fn test_sample_values() {
        assert_eq!(
            HashSet::from([
                Sut::sample_alice(),
                Sut::sample_bob(),
                Sut::sample_carol(),
                Sut::sample_dave(),
                Sut::sample_erin(),
                // duplicate should be removed
                Sut::sample_alice(),
                Sut::sample_bob(),
                Sut::sample_carol(),
                Sut::sample_dave(),
                Sut::sample_erin(),
            ])
            .len(),
            5
        )
    }
}
