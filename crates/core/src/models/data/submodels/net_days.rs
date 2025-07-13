use crate::prelude::*;

/// A typical payment terms structure that includes net payment due in a
/// specified number of days, e.g. `Net 30`.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    Builder,
    Getters,
    Display,
)]
#[display("Net {}", due_in)]
pub struct NetDays {
    /// The number of days until payment is due
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

impl HasSample for NetDays {
    fn sample() -> Self {
        Self::builder().due_in(Day::sample()).build()
    }
    fn sample_other() -> Self {
        Self::builder().due_in(Day::sample_other()).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = NetDays;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
