use crate::prelude::*;

/// Represents the fees for a consulting service, including the name, rate,
/// and billing cadence.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Getters, WithSetters)]
pub struct ServiceFees {
    /// Description of the consulting service, e.g. `"Agreed Consulting Fees"`
    #[getset(get = "pub", set_with = "pub")]
    name: String,

    /// The invoice rate
    #[getset(get = "pub", set_with = "pub")]
    rate: Rate,

    /// How often you invoice, cannot be
    #[getset(get = "pub")]
    cadence: Cadence,
}

#[bon]
impl ServiceFees {
    #[builder]
    pub fn new(name: String, rate: impl Into<Rate>, cadence: Cadence) -> Result<Self, Error> {
        let rate = rate.into();
        cadence.validate(rate.granularity())?;
        Ok(Self {
            name,
            rate,
            cadence,
        })
    }
}

impl ServiceFees {
    pub fn unit_price(&self) -> UnitPrice {
        self.rate.unit_price()
    }
}

impl HasSample for ServiceFees {
    fn sample() -> Self {
        Self::builder()
            .name("Discreet Investigative Services".to_string())
            .rate(Rate::daily(dec!(777.0)))
            .cadence(Cadence::Monthly)
            .build()
            .expect("Sample should be valid")
    }

    fn sample_other() -> Self {
        Self::builder()
            .name("Consulting Services".to_string())
            .rate(Rate::hourly(dec!(150.0)))
            .cadence(Cadence::BiWeekly)
            .build()
            .expect("Sample should be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    use test_log::test;

    type Sut = ServiceFees;

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
    fn test_serde() {
        assert_ron_snapshot!(Sut::sample())
    }
}
