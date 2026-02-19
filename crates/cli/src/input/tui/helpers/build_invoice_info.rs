use inquire::{CustomType, error::InquireResult};

use crate::{
    Cadence, FooterText, HexColor, InvoiceDataFromTuiError, InvoiceNumber, PeriodAnno,
    ProtoInvoiceInfo, PurchaseOrder, Result, TimestampedInvoiceNumber, WithOptionalDefault,
    YearAndMonth, build_period, format_help_skippable,
};

pub fn build_invoice_info(
    default: &ProtoInvoiceInfo<PeriodAnno>,
    cadence: Cadence,
) -> Result<ProtoInvoiceInfo<PeriodAnno>> {
    fn inner(
        default: &ProtoInvoiceInfo<PeriodAnno>,
        cadence: Cadence,
    ) -> InquireResult<ProtoInvoiceInfo<PeriodAnno>> {
        let invoice_number_offset = CustomType::<InvoiceNumber>::new(
            "What is the last invoice number you issued? We call this the 'offset'",
        )
        .with_help_message(&format_help_skippable(
            "Used with the date of that invoice to calculate future invoice numbers.".to_owned(),
        ))
        .with_default(default.offset().offset().clone())
        .prompt_skippable()?
        .unwrap_or_default();

        let invoice_number_offset_period = build_period(
            "When was that invoice issued? (Used to calculate future invoice numbers)".to_owned(),
            Some(default.offset().period().clone()),
            cadence,
        )?
        // if we use `0` as offset and set month to last month, then the next invoice number will be `1` for this month, which is correct.
        .unwrap_or(YearAndMonth::last().into());

        let offset = TimestampedInvoiceNumber::<PeriodAnno>::builder()
            .offset(invoice_number_offset)
            .period(invoice_number_offset_period)
            .build();

        let purchase_order = CustomType::<PurchaseOrder>::new("Purchase order number (optional)")
            .with_optional_default(default.purchase_order())
            .with_help_message(&format_help_skippable(
                "If you have a purchase order number, enter it here".to_owned(),
            ))
            .prompt_skippable()?;

        let footer_text = CustomType::<FooterText>::new("Footer text (optional)")
            .with_help_message(&format_help_skippable(
                "This is shown in the bottom of the invoice, it can e.g. be 'Reverse Charge'"
                    .to_owned(),
            ))
            .with_default(FooterText::default())
            .prompt_skippable()?;

        let emphasize_color_hex = CustomType::<HexColor>::new("Emphasize color (optional)")
            .with_optional_default(default.emphasize_color_hex())
            .with_help_message(&format_help_skippable(
                "This is used to emphasize certain parts of the invoice, e.g. '#e6007a'".to_owned(),
            ))
            .prompt_skippable()?;

        let info = ProtoInvoiceInfo::builder()
            .offset(offset)
            .maybe_purchase_order(purchase_order)
            .maybe_footer_text(footer_text)
            .maybe_emphasize_color_hex(emphasize_color_hex)
            .record_of_periods_off(default.record_of_periods_off().clone())
            .build();

        Ok(info)
    }
    inner(default, cadence)
        .map_err(InvoiceDataFromTuiError::invalid_invoice_info)
        .map_err(crate::Error::from)
}
