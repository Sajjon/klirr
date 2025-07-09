use crate::prelude::*;

/// Partial information about the invoice which can be used to derive a [`InvoiceInfoFull`]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Builder, Getters, Setters)]
pub struct ProtoInvoiceInfo {
    /// An offset which is used to calculate the invoice number, e.g. `(237, 2025-05)`.
    /// This is enables us to calculate the next invoice number based on the current
    /// date and this offset.
    #[getset(get = "pub", set_with = "pub")]
    offset: TimestampedInvoiceNumber,

    /// Record of months when we were 100% off, i.e. did not invoice for, e.g. `["2025-01", "2025-02"]`.
    #[builder(default)]
    #[getset(get = "pub", set = "pub")]
    months_off_record: MonthsOffRecord,

    /// A purchase order number associated with this invoice, e.g. `"PO-12345"`
    /// Typically agreed upon between the vendor and client before the
    /// invoice is issued.
    #[getset(get = "pub", set_with = "pub")]
    purchase_order: Option<PurchaseOrder>,

    /// E.g. "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation."
    #[getset(get = "pub", set_with = "pub")]
    footer_text: Option<FooterText>,

    /// Hex color code for the color emphasis of the invoice, e.g. `"#e6007a"`.
    #[getset(get = "pub", set_with = "pub")]
    emphasize_color_hex: Option<HexColor>,
}

impl ProtoInvoiceInfo {
    /// Inserts a new month into the months off record.
    /// This is used to keep track of months when no invoices were issued.
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let mut invoice_info = ProtoInvoiceInfo::sample();
    /// let month = YearAndMonth::may(2025);
    /// invoice_info.insert_month_off(month);
    /// assert!(invoice_info.months_off_record().contains(&month));
    /// ```
    pub fn insert_month_off(&mut self, month: YearAndMonth) {
        let mut months_off = self.months_off_record.clone();
        months_off.insert(month);
        self.set_months_off_record(months_off);
    }
}
impl HasSample for ProtoInvoiceInfo {
    fn sample() -> Self {
        Self::builder()
            .purchase_order(PurchaseOrder::sample())
            .footer_text(FooterText::sample())
            .emphasize_color_hex(HexColor::sample())
            .offset(TimestampedInvoiceNumber::sample())
            .months_off_record(MonthsOffRecord::sample())
            .build()
    }
}

impl ProtoInvoiceInfo {
    /// Validates the invoice information, ensuring that the offset month
    /// is not in the record of months off.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let invoice_info = ProtoInvoiceInfo::sample();
    /// assert!(invoice_info.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.months_off_record.contains(self.offset.month()) {
            return Err(Error::OffsetMonthMustNotBeInRecordOfMonthsOff {
                offset_month: *self.offset.month(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_log::test;

    #[test]
    fn test_advance() {
        let date = Date::from_str("2025-05-31").unwrap();
        let advanced = date.advance(&PaymentTerms::net30());
        assert_eq!(advanced, Date::from_str("2025-06-30").unwrap());
    }

    #[test]
    fn test_proto_invoice_info_validate_valid() {
        let invoice_info = ProtoInvoiceInfo::sample();
        assert!(invoice_info.validate().is_ok());
    }

    #[test]
    fn test_proto_invoice_info_validate_invalid() {
        let month = YearAndMonth::may(2025);
        let invoice_info = ProtoInvoiceInfo::builder()
            .offset(
                TimestampedInvoiceNumber::builder()
                    .month(month)
                    .offset(237.into())
                    .build(),
            )
            .months_off_record(MonthsOffRecord::new([month]))
            .purchase_order(PurchaseOrder::sample())
            .build();
        let result = invoice_info.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_month_off() {
        let mut invoice_info = ProtoInvoiceInfo::sample();
        let month = YearAndMonth::may(2025);
        invoice_info.insert_month_off(month);
        assert!(invoice_info.months_off_record.contains(&month));
    }
}
