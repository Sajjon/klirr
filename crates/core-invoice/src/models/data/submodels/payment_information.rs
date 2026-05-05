use crate::{Currency, HasSample, PaymentTerms};
use bon::Builder;
use getset::Getters;
use getset::WithSetters;
use klirr_foundation::Vat;
use serde::Deserialize;
use serde::Serialize;

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
}
