use crate::{Cadence, Error, HasSample, Rate, Result, UnitPrice};
use bon::bon;
use getset::Getters;
use getset::WithSetters;
use rust_decimal::dec;
use serde::Deserialize;
use serde::Serialize;

/// Represents the fees for a consulting service, including the name, rate,
/// and billing cadence.
///
/// **The `rate` is VAT-exclusive.** Any VAT is configured separately on
/// [`crate::PaymentInformation::vat`] and applied to the resulting subtotal at
/// render time, never embedded in the unit price stored here.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Getters, WithSetters)]
pub struct ServiceFees {
    /// Description of the consulting service, e.g. `"Agreed Consulting Fees"`
    #[getset(get = "pub", set_with = "pub")]
    name: String,

    /// The invoice rate, **excluding VAT**. VAT — if any — is configured on
    /// [`crate::PaymentInformation::vat`] and added on top of the computed
    /// subtotal when the invoice is rendered.
    #[getset(get = "pub", set_with = "pub")]
    rate: Rate,

    /// How often you invoice, cannot be
    #[getset(get = "pub")]
    cadence: Cadence,

    /// When `true`, public holidays in the vendor's country are deducted from
    /// billable working days (for day- and hour-granularity rates). Defaults to
    /// `false`, which preserves the prior behavior of counting every weekday.
    ///
    /// Holidays are looked up online (and cached to disk) using the vendor's
    /// country; if the country cannot be resolved or the lookup fails, no
    /// deduction is made.
    #[getset(get = "pub")]
    #[serde(default)]
    off_on_bank_holidays: bool,
}

#[bon]
impl ServiceFees {
    #[builder]
    pub fn new(
        name: impl AsRef<str>,
        rate: impl Into<Rate>,
        cadence: Cadence,
        #[builder(default)] off_on_bank_holidays: bool,
    ) -> Result<Self, Error> {
        let rate = rate.into();
        if !cadence.validate(rate.granularity()) {
            return Err(Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly);
        }
        Ok(Self {
            name: name.as_ref().to_owned(),
            rate,
            cadence,
            off_on_bank_holidays,
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
    use crate::HasSample;
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

    #[test]
    fn off_on_bank_holidays_defaults_to_false() {
        assert!(!Sut::sample().off_on_bank_holidays());
    }

    #[test]
    fn builder_sets_off_on_bank_holidays() {
        let fees = Sut::builder()
            .name("Consulting".to_string())
            .rate(crate::Rate::daily(dec!(100.0)))
            .cadence(crate::Cadence::Monthly)
            .off_on_bank_holidays(true)
            .build()
            .unwrap();
        assert!(fees.off_on_bank_holidays());
    }

    #[test]
    fn deserializes_legacy_ron_without_flag_as_false() {
        // RON persisted before the field existed must still load (serde default).
        let legacy = r#"(
            name: "Agreed Consulting Service",
            rate: Daily(UnitPrice(777.0)),
            cadence: Monthly,
        )"#;
        let fees: Sut = crate::deserialize_ron_str(legacy).unwrap();
        assert!(!fees.off_on_bank_holidays());
        assert_eq!(fees.name(), "Agreed Consulting Service");
    }
}
