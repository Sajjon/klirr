use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct InvoiceInfoFull {
    /// The unique number of this invoice, typically a number, e.g. `"90"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    number: InvoiceNumber,

    /// When the payment is due, calculated from the invoice date and payment terms.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    invoice_date: Date,

    /// When the payment is due, calculated from the invoice date and payment terms.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    due_date: Date,

    /// A purchase order number associated with this invoice, e.g. `"PO-12345"`
    /// Typically agreed upon between the vendor and client before the
    /// invoice is issued.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    purchase_order: PurchaseOrder,

    /// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    terms: PaymentTerms,

    /// E.g. "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation."
    #[builder(setter(into))]
    #[getset(get = "pub")]
    footer_text: String,

    /// Hex color code for the color emphasis of the invoice, e.g. `"#e6007a"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    emphasize_color_hex: HexColor,
}

impl InvoiceInfoFull {
    pub fn sample() -> Self {
        Self::builder()
            .number(InvoiceNumber::sample())
            .invoice_date(Date::sample())
            .due_date(Date::sample())
            .purchase_order(PurchaseOrder::sample())
            .terms(PaymentTerms::sample())
            .footer_text(
                "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation."
                    .to_string(),
            )
            .emphasize_color_hex(HexColor::sample())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_invoice_info_full_sample() {
        let sample = InvoiceInfoFull::sample();
        assert!(!sample.footer_text().is_empty());
    }
}
