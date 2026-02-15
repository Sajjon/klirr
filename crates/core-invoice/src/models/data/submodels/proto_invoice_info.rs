use crate::prelude::*;

/// Partial information about the invoice which can be used to derive a [`InvoiceInfoFull`]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Builder, Getters, Setters)]
pub struct ProtoInvoiceInfo<Period: IsPeriod> {
    /// An offset which is used to calculate the invoice number, e.g. `(237, 2025-05)`.
    /// This is enables us to calculate the next invoice number based on the current
    /// date and this offset.
    #[getset(get = "pub", set_with = "pub")]
    offset: TimestampedInvoiceNumber<Period>,

    /// Record of periods when we were 100% off, i.e. did not invoice for, e.g. `["2025-01", "2025-02"]`.
    #[builder(default)]
    #[getset(get = "pub", set = "pub")]
    record_of_periods_off: RecordOfPeriodsOff<Period>,

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

impl<Period: IsPeriod> ProtoInvoiceInfo<Period> {
    /// Inserts a new month into the months off record.
    /// This is used to keep track of months when no invoices were issued.
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let mut invoice_info = ProtoInvoiceInfo::<YearAndMonth>::sample();
    /// let month = YearAndMonth::may(2025);
    /// invoice_info.insert_period_off(month);
    /// assert!(invoice_info.record_of_periods_off().contains(&month));
    /// ```
    pub fn insert_period_off(&mut self, period: Period) {
        let mut periods_off = self.record_of_periods_off.clone();
        periods_off.insert(period);
        self.set_record_of_periods_off(periods_off);
    }
}

impl<Period: IsPeriod + HasSample> HasSample for ProtoInvoiceInfo<Period> {
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

impl<Period: IsPeriod> ProtoInvoiceInfo<Period> {
    /// Validates the invoice information, ensuring that the offset month
    /// is not in the record of months off.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let invoice_info = ProtoInvoiceInfo::<YearAndMonth>::sample();
    /// assert!(invoice_info.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.record_of_periods_off.contains(self.offset.period()) {
            return Err(Error::OffsetPeriodMustNotBeInRecordOfPeriodsOff {
                offset_period: format!("{:?}", self.offset.period()),
                period_kind: type_name::<Period>(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_log::test;

    type Sut = ProtoInvoiceInfo<YearAndMonth>;

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

    #[test]
    fn test_proto_invoice_info_validate_valid() {
        let invoice_info = Sut::sample();
        assert!(invoice_info.validate().is_ok());
    }

    #[test]
    fn test_proto_invoice_info_validate_invalid() {
        let month = YearAndMonth::may(2025);
        let invoice_info = Sut::builder()
            .offset(
                TimestampedInvoiceNumber::builder()
                    .period(month)
                    .offset(237.into())
                    .build(),
            )
            .record_of_periods_off(RecordOfPeriodsOff::new([month]))
            .purchase_order(PurchaseOrder::sample())
            .build();
        let result = invoice_info.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_period_off() {
        let mut invoice_info = Sut::sample();
        let period = YearAndMonth::may(2025);
        invoice_info.insert_period_off(period);
        assert!(invoice_info.record_of_periods_off().contains(&period));
    }
}
