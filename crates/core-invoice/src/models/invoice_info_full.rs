use crate::{Date, FooterText, HasSample, HexColor, InvoiceNumber, PurchaseOrder};
use bon::Builder;
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;

/// A full invoice information structure that includes the derived
/// invoice number, invoice date, due date and other
/// details from the `ProtoInvoiceInfo`.
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash, Deserialize, Getters, Builder)]
pub struct InvoiceInfoFull {
    /// The unique number of this invoice, typically a number, e.g. `"90"`
    #[getset(get = "pub")]
    number: InvoiceNumber,

    /// When the payment is due, calculated from the invoice date and payment terms.
    #[getset(get = "pub")]
    invoice_date: Date,

    /// When the payment is due, calculated from the invoice date and payment terms.
    #[getset(get = "pub")]
    due_date: Date,

    /// A purchase order number associated with this invoice, e.g. `"PO-12345"`
    /// Typically agreed upon between the vendor and client before the
    /// invoice is issued.
    #[getset(get = "pub")]
    purchase_order: Option<PurchaseOrder>,

    /// E.g. "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation."
    #[getset(get = "pub")]
    footer_text: Option<FooterText>,

    /// Hex color code for the color emphasis of the invoice, e.g. `"#e6007a"`.
    #[builder(default)]
    #[getset(get = "pub")]
    emphasize_color_hex: HexColor,
}

impl HasSample for InvoiceInfoFull {
    fn sample() -> Self {
        Self::builder()
            .number(InvoiceNumber::sample())
            .invoice_date(Date::sample())
            .due_date(Date::sample())
            .purchase_order(PurchaseOrder::sample())
            .footer_text(FooterText::sample())
            .emphasize_color_hex(HexColor::sample())
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .number(InvoiceNumber::sample_other())
            .invoice_date(Date::sample_other())
            .due_date(Date::sample_other())
            .purchase_order(PurchaseOrder::sample_other())
            .footer_text(FooterText::sample_other())
            .emphasize_color_hex(HexColor::sample_other())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use test_log::test;

    type Sut = InvoiceInfoFull;

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
    fn test_invoice_info_full_sample() {
        let sample = Sut::sample();
        assert!(sample.footer_text().is_some());
    }
}
