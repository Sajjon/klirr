use crate::prelude::*;
use derive_more::FromStr;

/// Footer text for the invoice, e.g. "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation."
/// This is typically used to provide additional information about the invoice,
/// such as tax information or payment instructions.
#[derive(Clone, Debug, Display, Serialize, Deserialize, PartialEq, From, Deref, FromStr)]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn default_neq_sample() {
        let default = FooterText::default();
        let sample = FooterText::sample();
        assert_ne!(default, sample);
    }
}
