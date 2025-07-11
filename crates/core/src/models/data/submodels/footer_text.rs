use crate::prelude::*;
use derive_more::FromStr;

/// Footer text for the invoice, e.g. "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation."
/// This is typically used to provide additional information about the invoice,
/// such as tax information or payment instructions.
#[derive(
    Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq, Hash, From, Deref, FromStr,
)]
#[from(String, &'static str)]
#[serde(transparent)]
pub struct FooterText(String);

impl Default for FooterText {
    fn default() -> Self {
        Self::from("Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation.")
    }
}

impl HasSample for FooterText {
    fn sample() -> Self {
        Self::from("Billed with the utmost discretion—your secrets are safe, for a price.")
    }

    fn sample_other() -> Self {
        Self::from("Thank you for your business! We appreciate your trust in our services.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    type Sut = FooterText;

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
    fn default_neq_sample() {
        let default = Sut::default();
        let sample = Sut::sample();
        assert_ne!(default, sample);
    }
}
