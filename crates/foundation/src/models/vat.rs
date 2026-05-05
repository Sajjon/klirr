use crate::{Decimal, HasSample, ModelError, ModelResult};
use derive_more::Deref;
use derive_more::Display;
use rust_decimal::dec;
use rust_decimal::prelude::ToPrimitive;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

/// Value-Added Tax rate, expressed as a percentage (0–100) applied to the
/// invoice subtotal.
///
/// `Vat::from_percent(dec!(25))` represents 25% VAT. A rate of 0% (the default)
/// means no VAT row is rendered on the invoice.
///
/// **VAT is always added on top.** The configured service [`UnitPrice`] /
/// [`Rate`] / [`Cost`] are stored VAT-exclusive; VAT is computed as
/// `subtotal * percent / 100` at render time and added to produce the grand
/// total. Never embed VAT into the unit price.
///
/// [`UnitPrice`]: crate::UnitPrice
/// [`Rate`]: crate::Rate
/// [`Cost`]: crate::Cost
#[derive(
    Clone, Copy, Display, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Deref,
)]
#[deref(forward)]
#[serde(transparent)]
pub struct Vat(Decimal);

impl Vat {
    /// 0% VAT — when set, no VAT row is rendered.
    pub const ZERO: Self = Self(Decimal::ZERO);

    /// Constructs a `Vat` from a percentage value, e.g. `dec!(25)` for 25%.
    ///
    /// # Errors
    /// Returns an error if `percent` is negative or greater than `100`.
    ///
    /// # Examples
    /// ```
    /// use klirr_foundation::Vat;
    /// use rust_decimal::dec;
    ///
    /// let vat = Vat::from_percent(dec!(25)).unwrap();
    /// assert!(!vat.is_zero());
    ///
    /// assert!(Vat::from_percent(dec!(-1)).is_err());
    /// assert!(Vat::from_percent(dec!(101)).is_err());
    /// ```
    pub fn from_percent(percent: impl Into<Decimal>) -> ModelResult<Self> {
        let percent = percent.into();
        let raw: rust_decimal::Decimal = *percent;
        if raw.is_sign_negative() {
            return Err(ModelError::InvalidVatPercentage {
                percent: raw.to_f64().unwrap_or(f64::NAN),
                reason: "VAT percentage must not be negative".to_owned(),
            });
        }
        if raw > rust_decimal::Decimal::ONE_HUNDRED {
            return Err(ModelError::InvalidVatPercentage {
                percent: raw.to_f64().unwrap_or(f64::NAN),
                reason: "VAT percentage must not exceed 100".to_owned(),
            });
        }
        Ok(Self(percent))
    }

    /// Returns true if this VAT rate is exactly 0% — meaning the row should be
    /// hidden on the rendered invoice.
    ///
    /// # Examples
    /// ```
    /// use klirr_foundation::Vat;
    /// use rust_decimal::dec;
    ///
    /// assert!(Vat::ZERO.is_zero());
    /// assert!(Vat::default().is_zero());
    /// assert!(!Vat::from_percent(dec!(25)).unwrap().is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        *self.0 == rust_decimal::Decimal::ZERO
    }

    /// The percentage as a `Decimal` (e.g. `25.0` for 25%).
    ///
    /// # Examples
    /// ```
    /// use klirr_foundation::{Decimal, Vat};
    /// use rust_decimal::dec;
    ///
    /// let vat = Vat::from_percent(dec!(25)).unwrap();
    /// assert_eq!(*vat.percent(), dec!(25));
    /// ```
    pub fn percent(&self) -> Decimal {
        self.0
    }
}

impl FromStr for Vat {
    type Err = ModelError;

    fn from_str(s: &str) -> ModelResult<Self> {
        let trimmed = s.trim().trim_end_matches('%').trim();
        let decimal = rust_decimal::Decimal::from_str(trimmed).map_err(|underlying| {
            ModelError::InvalidVatPercentageFromString {
                invalid_string: s.to_owned(),
                reason: underlying.to_string(),
            }
        })?;
        Self::from_percent(Decimal::from(decimal))
    }
}

impl HasSample for Vat {
    fn sample() -> Self {
        Self::from_percent(dec!(25)).expect("25% is a valid VAT rate")
    }

    fn sample_other() -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;
    use test_log::test;

    type Sut = Vat;

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
    fn default_is_zero() {
        assert!(Sut::default().is_zero());
        assert_eq!(Sut::default(), Sut::ZERO);
    }

    #[test]
    fn sample_other_is_zero() {
        assert!(Sut::sample_other().is_zero());
    }

    #[test]
    fn sample_is_25_percent() {
        assert_eq!(*Sut::sample().percent(), dec!(25));
        assert!(!Sut::sample().is_zero());
    }

    #[test]
    fn from_percent_rejects_negative() {
        let result = Vat::from_percent(dec!(-1));
        assert!(matches!(
            result,
            Err(ModelError::InvalidVatPercentage { .. })
        ));
    }

    #[test]
    fn from_percent_rejects_above_100() {
        let result = Vat::from_percent(dec!(100.01));
        assert!(matches!(
            result,
            Err(ModelError::InvalidVatPercentage { .. })
        ));
    }

    #[test]
    fn from_percent_accepts_boundary_zero_and_hundred() {
        assert!(Vat::from_percent(dec!(0)).is_ok());
        assert!(Vat::from_percent(dec!(100)).is_ok());
    }

    #[test]
    fn from_str_with_percent_sign() {
        let parsed: Vat = "25%".parse().unwrap();
        assert_eq!(parsed, Sut::sample());
    }

    #[test]
    fn from_str_without_percent_sign() {
        let parsed: Vat = "25".parse().unwrap();
        assert_eq!(parsed, Sut::sample());
    }

    #[test]
    fn from_str_invalid() {
        let parsed: Result<Vat, _> = "abc".parse();
        assert!(matches!(
            parsed,
            Err(ModelError::InvalidVatPercentageFromString { .. })
        ));
    }

    #[test]
    fn from_str_negative() {
        let parsed: Result<Vat, _> = "-5%".parse();
        assert!(matches!(
            parsed,
            Err(ModelError::InvalidVatPercentage { .. })
        ));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Sut::sample()), "25");
        assert_eq!(format!("{}", Sut::ZERO), "0");
    }

    #[test]
    fn test_serde() {
        assert_ron_snapshot!(Sut::sample());
    }

    #[test]
    fn serde_roundtrip_zero() {
        let zero = Vat::ZERO;
        let json = serde_json::to_string(&zero).unwrap();
        let back: Vat = serde_json::from_str(&json).unwrap();
        assert_eq!(back, zero);
    }
}
