use crate::{
    Error, FooterText, HasSample, HexColor, PurchaseOrder, RecordOfPeriodsOff, Result,
    TimestampedInvoiceNumber,
};
use bon::Builder;
use getset::{Getters, Setters, WithSetters};
use serde::{Deserialize, Serialize};

/// Partial information about the invoice which can be used to derive [`crate::InvoiceInfoFull`].
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Builder, Getters, Setters, WithSetters,
)]
pub struct ProtoInvoiceInfo {
    /// An offset used to calculate the invoice number, e.g. `(237, 2025-05-31)`.
    /// This enables computing later invoice numbers from the stored reference point.
    #[getset(get = "pub", set_with = "pub")]
    offset: TimestampedInvoiceNumber,

    /// Period-end dates when no invoice was issued, i.e. full periods off.
    #[builder(default)]
    #[getset(get = "pub", set = "pub")]
    record_of_periods_off: RecordOfPeriodsOff,

    /// A purchase order number associated with the invoice, e.g. `"PO-12345"`.
    /// This is typically agreed between vendor and client before invoicing.
    #[getset(get = "pub", set_with = "pub")]
    purchase_order: Option<PurchaseOrder>,

    /// Optional legal or contractual footer text printed on the invoice.
    /// Example: reverse VAT notice text.
    #[getset(get = "pub", set_with = "pub")]
    footer_text: Option<FooterText>,

    /// Hex color used for invoice emphasis styling, e.g. `"#e6007a"`.
    #[getset(get = "pub", set_with = "pub")]
    emphasize_color_hex: Option<HexColor>,
}

impl ProtoInvoiceInfo {
    pub fn insert_period_off(&mut self, period_end_date: crate::Date) {
        let mut periods_off = self.record_of_periods_off.clone();
        periods_off.insert(period_end_date);
        self.set_record_of_periods_off(periods_off);
    }
}

impl HasSample for ProtoInvoiceInfo {
    fn sample() -> Self {
        Self::builder()
            .purchase_order(PurchaseOrder::sample())
            .footer_text(FooterText::sample())
            .emphasize_color_hex(HexColor::sample())
            .offset(TimestampedInvoiceNumber::sample())
            .record_of_periods_off(RecordOfPeriodsOff::default())
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .purchase_order(PurchaseOrder::sample_other())
            .footer_text(FooterText::sample_other())
            .emphasize_color_hex(HexColor::sample_other())
            .offset(TimestampedInvoiceNumber::sample_other())
            .record_of_periods_off(RecordOfPeriodsOff::default())
            .build()
    }
}

impl ProtoInvoiceInfo {
    pub fn validate(&self) -> Result<()> {
        if self.record_of_periods_off.contains(self.offset.date()) {
            return Err(Error::OffsetPeriodMustNotBeInRecordOfPeriodsOff {
                offset_period: format!("{:?}", self.offset.date()),
                period_kind: "Date".to_owned(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Date, PaymentTerms};
    use std::str::FromStr;

    type Sut = ProtoInvoiceInfo;

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
    fn test_advance() {
        let date = Date::from_str("2025-05-31").unwrap();
        let advanced = date.advance(&PaymentTerms::net30());
        assert_eq!(advanced, Date::from_str("2025-06-30").unwrap());
    }
}
