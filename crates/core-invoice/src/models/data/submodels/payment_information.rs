use crate::{Currency, Error, HasSample, LabeledField, PaymentTerms, Result};
use bon::Builder;
use getset::Getters;
use getset::WithSetters;
use klirr_foundation::Vat;
use serde::Deserialize;
use serde::Serialize;

/// Maximum number of [`LabeledField`] entries accepted in
/// [`PaymentInformation::payment_method_overrides`]. The Typst layout has
/// exactly two slots (the IBAN slot and the BIC slot) that overrides can
/// occupy.
pub const MAX_PAYMENT_METHOD_OVERRIDES: usize = 2;

/// Bank account details for the vendor, used for international transfers.
/// This includes the IBAN, bank name, and BIC.
/// This is used to ensure that the client can pay the invoice correctly.
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Builder, Getters, WithSetters,
)]
pub struct PaymentInformation {
    /// The IBAN (International Bank Account Number) of the vendor's bank account,
    #[getset(get = "pub", set_with = "pub")]
    iban: String,

    /// The name of the vendor's bank, used for international transfers.
    #[getset(get = "pub", set_with = "pub")]
    bank_name: String,

    /// The BIC (Bank Identifier Code) of the vendor's bank, used for international
    #[getset(get = "pub", set_with = "pub")]
    bic: String,

    /// The currency of this invoice, e.g. `EUR`
    #[getset(get = "pub", set_with = "pub")]
    currency: Currency,

    /// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
    #[getset(get = "pub", set_with = "pub")]
    terms: PaymentTerms,

    /// VAT rate applied **on top of** the VAT-exclusive invoice subtotal.
    ///
    /// The subtotal is the sum of each line item's `quantity * unit_price`
    /// where unit prices are stored VAT-exclusive (see [`crate::ServiceFees`]
    /// and [`klirr_foundation::UnitPrice`]). VAT is computed as
    /// `subtotal * percent / 100` and added to the subtotal to produce the
    /// grand total. When `0%` (the default), no VAT row is rendered and the
    /// grand total equals the subtotal.
    #[builder(default)]
    #[serde(default)]
    #[getset(get = "pub", set_with = "pub")]
    vat: Vat,

    /// Optional payment-method overrides shown in the rendered footer in
    /// place of the IBAN / BIC values. `iban` and `bic` remain in the data
    /// model so they can still be sent or persisted, but they are *not*
    /// rendered when overridden.
    ///
    /// Slot mapping in the rendered middle column (top → bottom):
    ///
    /// 1. Bank name (always shown).
    /// 2. **IBAN slot** — replaced when `payment_method_overrides.len() == 2`
    ///    by `payment_method_overrides[0]`.
    /// 3. **BIC slot** — replaced when `payment_method_overrides.len() >= 1`
    ///    by `payment_method_overrides.last()`.
    ///
    /// Use cases: swapping IBAN+BIC for a Swedish `Bankgiro` + `Kontonummer`
    /// pair, or showing a single `Plusgiro` line in place of the BIC.
    ///
    /// Validation: at most [`MAX_PAYMENT_METHOD_OVERRIDES`] entries.
    /// `#[serde(default)]` keeps existing RON files (written before this
    /// field existed) deserializable with no overrides.
    #[builder(default)]
    #[serde(default)]
    #[getset(get = "pub")]
    payment_method_overrides: Vec<LabeledField>,
}

impl PaymentInformation {
    /// Validates the [`PaymentInformation`] invariants and returns `self` when
    /// valid. Currently checks that at most
    /// [`MAX_PAYMENT_METHOD_OVERRIDES`] override entries are present.
    ///
    /// # Errors
    /// Returns [`Error::TooManyPaymentMethodOverrides`] when
    /// `payment_method_overrides.len() > MAX_PAYMENT_METHOD_OVERRIDES`.
    pub fn validate(self) -> Result<Self> {
        if self.payment_method_overrides.len() > MAX_PAYMENT_METHOD_OVERRIDES {
            return Err(Error::TooManyPaymentMethodOverrides {
                found: self.payment_method_overrides.len(),
                max: MAX_PAYMENT_METHOD_OVERRIDES,
            });
        }
        Ok(self)
    }

    /// Replaces the override list, validating the new size.
    ///
    /// # Errors
    /// Returns [`Error::TooManyPaymentMethodOverrides`] when more than
    /// [`MAX_PAYMENT_METHOD_OVERRIDES`] entries are supplied.
    pub fn with_payment_method_overrides(mut self, overrides: Vec<LabeledField>) -> Result<Self> {
        self.payment_method_overrides = overrides;
        self.validate()
    }
}

impl HasSample for PaymentInformation {
    fn sample() -> Self {
        Self::builder()
            .bank_name("Banque de Paris".into())
            .iban("FR76 3000 6000 0112 3456 7890 189".into())
            .bic("BNPAFRPP".into())
            .currency(Currency::EUR)
            .terms(PaymentTerms::sample())
            .vat(Vat::ZERO)
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .bank_name("Bank of London".into())
            .iban("GB29 NWBK 6016 1331 9268 19".into())
            .bic("NWBKGB2L".into())
            .currency(Currency::USD)
            .terms(PaymentTerms::sample_other())
            .vat(Vat::sample())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use rust_decimal::dec;

    type Sut = PaymentInformation;

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
    fn sample_defaults_to_zero_vat() {
        assert_eq!(*Sut::sample().vat(), Vat::ZERO);
    }

    #[test]
    fn sample_other_has_non_zero_vat() {
        assert!(!Sut::sample_other().vat().is_zero());
    }

    #[test]
    fn with_vat_overrides_field() {
        let vat = Vat::from_percent(dec!(12)).unwrap();
        let sut = Sut::sample().with_vat(vat);
        assert_eq!(*sut.vat(), vat);
    }

    /// `#[serde(default)]` on `vat` lets older RON files (written before VAT
    /// was added) deserialize cleanly with `Vat::ZERO`.
    #[test]
    fn deserializes_with_default_vat_when_field_missing() {
        let ron_without_vat = r#"PaymentInformation(
            iban: "FR76 3000 6000 0112 3456 7890 189",
            bank_name: "Banque de Paris",
            bic: "BNPAFRPP",
            currency: "EUR",
            terms: "Net 30",
        )"#;
        let parsed: PaymentInformation = ron::from_str(ron_without_vat).unwrap();
        assert_eq!(*parsed.vat(), Vat::ZERO);
    }

    /// Builder uses `#[builder(default)]` for `vat`, so existing call sites
    /// that didn't set VAT must still build successfully and produce `0%`.
    #[test]
    fn builder_default_vat_is_zero() {
        let pi = PaymentInformation::builder()
            .bank_name("Bank".into())
            .iban("IBAN".into())
            .bic("BIC".into())
            .currency(Currency::EUR)
            .terms(PaymentTerms::sample())
            .build();
        assert_eq!(*pi.vat(), Vat::ZERO);
    }

    #[test]
    fn samples_have_no_payment_method_overrides() {
        assert!(Sut::sample().payment_method_overrides().is_empty());
        assert!(Sut::sample_other().payment_method_overrides().is_empty());
    }

    #[test]
    fn deserializes_with_default_overrides_when_field_missing() {
        let ron = r#"PaymentInformation(
            iban: "FR76 3000 6000 0112 3456 7890 189",
            bank_name: "Banque de Paris",
            bic: "BNPAFRPP",
            currency: "EUR",
            terms: "Net 30",
        )"#;
        let parsed: PaymentInformation = ron::from_str(ron).unwrap();
        assert!(parsed.payment_method_overrides().is_empty());
    }

    #[test]
    fn with_payment_method_overrides_accepts_zero_one_or_two() {
        let base = Sut::sample();
        assert!(base.clone().with_payment_method_overrides(vec![]).is_ok());
        assert!(
            base.clone()
                .with_payment_method_overrides(vec![LabeledField::sample()])
                .is_ok()
        );
        assert!(
            base.clone()
                .with_payment_method_overrides(vec![
                    LabeledField::sample(),
                    LabeledField::sample_other(),
                ])
                .is_ok()
        );
    }

    #[test]
    fn with_payment_method_overrides_rejects_more_than_max() {
        let three = vec![
            LabeledField::sample(),
            LabeledField::sample_other(),
            LabeledField::new("Plusgiro", "12-3456-7"),
        ];
        let result = Sut::sample().with_payment_method_overrides(three);
        assert!(matches!(
            result,
            Err(Error::TooManyPaymentMethodOverrides { found: 3, max: 2 })
        ));
    }

    #[test]
    fn validate_rejects_too_many_overrides_when_built_directly() {
        let pi = PaymentInformation::builder()
            .bank_name("Bank".into())
            .iban("IBAN".into())
            .bic("BIC".into())
            .currency(Currency::EUR)
            .terms(PaymentTerms::sample())
            .payment_method_overrides(vec![
                LabeledField::sample(),
                LabeledField::sample_other(),
                LabeledField::new("Plusgiro", "12-3456-7"),
            ])
            .build();
        assert!(matches!(
            pi.validate(),
            Err(Error::TooManyPaymentMethodOverrides { found: 3, max: 2 })
        ));
    }

    #[test]
    fn validate_passes_with_zero_one_or_two_overrides() {
        let valid = PaymentInformation::builder()
            .bank_name("Bank".into())
            .iban("IBAN".into())
            .bic("BIC".into())
            .currency(Currency::EUR)
            .terms(PaymentTerms::sample())
            .payment_method_overrides(vec![LabeledField::sample(), LabeledField::sample_other()])
            .build();
        assert!(valid.validate().is_ok());
    }
}
