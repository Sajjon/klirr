use crate::HasSample;
use bon::Builder;
use getset::{Getters, WithSetters};
use serde::{Deserialize, Serialize};

/// A free-form `(label, value)` pair, neither field validated.
///
/// Used as a payment-method override entry on
/// [`crate::PaymentInformation::payment_method_overrides`] so vendors can show
/// non-IBAN/BIC routing details (Bankgiro, Plusgiro, account numbers, …) on
/// the rendered invoice without coercing them into the IBAN/BIC slots in the
/// data model. Both `label` and `value` are passed through verbatim.
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Builder, Getters, WithSetters,
)]
pub struct LabeledField {
    /// The label printed in the footer column, e.g. `"Bankgiro"`.
    #[getset(get = "pub", set_with = "pub")]
    label: String,

    /// The value printed under the label, e.g. `"153-3827"`.
    #[getset(get = "pub", set_with = "pub")]
    value: String,
}

impl LabeledField {
    /// Convenience constructor accepting any `Into<String>` pair.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::LabeledField;
    ///
    /// let field = LabeledField::new("Bankgiro", "153-3827");
    /// assert_eq!(field.label(), "Bankgiro");
    /// assert_eq!(field.value(), "153-3827");
    /// ```
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

impl HasSample for LabeledField {
    fn sample() -> Self {
        Self::new("Reference", "INV-SAMPLE-001")
    }

    fn sample_other() -> Self {
        Self::new("Cost Center", "CC-42-EU")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = LabeledField;

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
    fn new_accepts_string_and_str() {
        let from_str = Sut::new("a", "b");
        let from_string = Sut::new(String::from("a"), String::from("b"));
        assert_eq!(from_str, from_string);
    }

    #[test]
    fn builder_constructs_field() {
        let field = LabeledField::builder()
            .label("Plusgiro".into())
            .value("12-3456-7".into())
            .build();
        assert_eq!(field.label(), "Plusgiro");
        assert_eq!(field.value(), "12-3456-7");
    }

    #[test]
    fn with_setters_replace_fields() {
        let updated = Sut::sample()
            .with_label("Plusgiro".into())
            .with_value("12-3456-7".into());
        assert_eq!(updated.label(), "Plusgiro");
        assert_eq!(updated.value(), "12-3456-7");
    }

    #[test]
    fn label_and_value_are_passed_through_verbatim() {
        // No validation — punctuation, whitespace, non-ASCII all preserved.
        let field = Sut::new("  Möms — note ", "  not, validated\n");
        assert_eq!(field.label(), "  Möms — note ");
        assert_eq!(field.value(), "  not, validated\n");
    }

    #[test]
    fn serde_roundtrip() {
        let field = Sut::sample();
        let json = serde_json::to_string(&field).unwrap();
        let back: LabeledField = serde_json::from_str(&json).unwrap();
        assert_eq!(back, field);
    }

    #[test]
    fn deserializes_from_ron() {
        let parsed: LabeledField =
            ron::from_str(r#"LabeledField(label: "Reference", value: "INV-SAMPLE-001")"#).unwrap();
        assert_eq!(parsed, Sut::sample());
    }
}
