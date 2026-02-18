use crate::{Day, FromStr, HasSample, NetDays, Result};
use derive_more::Display;
use serde::Deserialize;
use serde::Serialize;

/// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
#[derive(Clone, Debug, Display, Serialize, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
pub enum PaymentTerms {
    /// Net payment due in a specific number of days, e.g. `Net(30)`
    Net(NetDays),
}

impl FromStr for PaymentTerms {
    type Err = crate::Error;

    /// Parses a string into `PaymentTerms`, e.g. "Net 30" into `PaymentTerms::Net(NetDays { due_in: 30 })`.
    /// # Errors
    /// Returns an error if the string is not in the correct format or if
    /// the number of days is invalid.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::*;
    /// let payment_terms: PaymentTerms = "Net 30".parse().unwrap();
    /// assert!(matches!(payment_terms, PaymentTerms::Net(_)));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let net_days = NetDays::from_str(s)?;
        Ok(PaymentTerms::Net(net_days))
    }
}

impl Default for PaymentTerms {
    fn default() -> Self {
        Self::net30()
    }
}

impl PaymentTerms {
    /// Creates a new `PaymentTerms` with net payment due in 30 days.
    pub fn net30() -> Self {
        PaymentTerms::Net(NetDays::net30())
    }
}

impl HasSample for PaymentTerms {
    fn sample() -> Self {
        Self::default()
    }

    fn sample_other() -> Self {
        Self::Net(
            NetDays::builder()
                .due_in(Day::try_from(15).unwrap())
                .build(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use insta::assert_ron_snapshot;

    type Sut = PaymentTerms;

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
    fn test_payment_terms_net_days() {
        let net_days: NetDays = "Net 30".parse().unwrap();
        assert_eq!(net_days.due_in(), &Day::try_from(30).unwrap());
        assert_ron_snapshot!(net_days);
    }

    #[test]
    fn test_payment_terms_default() {
        let payment_terms = Sut::default();
        assert!(matches!(payment_terms, Sut::Net(_)));
        assert_ron_snapshot!(payment_terms);
    }

    #[test]
    fn from_str_invalid_all_reasons() {
        let invalid_strings = [
            "Net",          // Missing days
            "Net 0",        // Invalid days (0)
            "Net -30",      // Invalid days (negative)
            "Net 32",       // Invalid days (more than 31)
            "Net abc",      // Non-numeric days
            "Net 30 extra", // Extra text after valid input
        ];

        for invalid in invalid_strings {
            let result: Result<NetDays, _> = invalid.parse();
            assert!(result.is_err(), "Expected error for '{}'", invalid);
        }
    }

    #[test]
    fn test_payment_terms_from_str() {
        let payment_terms: PaymentTerms = "Net 30".parse().unwrap();
        assert!(matches!(payment_terms, PaymentTerms::Net(_)));
    }
}
