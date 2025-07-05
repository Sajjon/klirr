use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::prelude::*;

/// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
#[derive(Clone, Debug, Display, Serialize, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
pub enum PaymentTerms {
    /// Net payment due in a specific number of days, e.g. `Net(30)`
    Net(NetDays),
}

impl FromStr for PaymentTerms {
    type Err = crate::prelude::Error;

    /// Parses a string into `PaymentTerms`, e.g. "Net 30" into `PaymentTerms::Net(NetDays { due_in: 30 })`.
    /// # Errors
    /// Returns an error if the string is not in the correct format or if
    /// the number of days is invalid.
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
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
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    TypedBuilder,
    Getters,
    Display,
)]
#[display("Net {}", due_in)]
pub struct NetDays {
    /// The number of days until payment is due
    #[builder(setter(into))]
    #[getset(get = "pub")]
    due_in: Day,
}
impl FromStr for NetDays {
    type Err = crate::prelude::Error;

    /// Tries to parse a string in the format "Net {days}", e.g. "Net 30".
    /// /// # Errors
    /// Returns an error if the string is not in the correct format or if
    /// the number of days is invalid.
    /// /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let net_days: NetDays = "Net 30".parse().unwrap();
    /// assert_eq!(net_days.due_in(), &Day::try_from(30).unwrap());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let days = s
            .split("Net ")
            .nth(1)
            .ok_or(Error::FailedToParsePaymentTermsNetDays {
                invalid_string: s.to_owned(),
            })?;
        let days = Day::from_str(days).map_err(|_| Error::FailedToParsePaymentTermsNetDays {
            invalid_string: s.to_owned(),
        })?;
        Ok(Self::builder().due_in(days).build())
    }
}

impl NetDays {
    pub fn net30() -> Self {
        Self::builder()
            .due_in(Day::try_from(30).expect("LEQ 31 days"))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_payment_terms_net_days() {
        let net_days: NetDays = "Net 30".parse().unwrap();
        assert_eq!(net_days.due_in(), &Day::try_from(30).unwrap());
        assert_ron_snapshot!(net_days);
    }

    #[test]
    fn test_payment_terms_default() {
        let payment_terms = PaymentTerms::default();
        assert!(matches!(payment_terms, PaymentTerms::Net(_)));
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
