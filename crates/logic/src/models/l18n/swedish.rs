use crate::prelude::*;

impl L18n {
    pub fn swedish() -> Self {
        Self::builder()
            .language(Language::SV)
            .content(L18nContent::swedish())
            .build()
    }
}

impl L18nClientInfo {
    pub fn swedish() -> Self {
        Self::builder()
            .to_company("Till:".to_string())
            .vat_number("Moms:".to_string())
            .build()
    }
}

impl L18nInvoiceInfo {
    pub fn swedish() -> Self {
        Self::builder()
            .purchase_order("Inköpsorder:".to_string())
            .invoice_identifier("Fakturanr:".to_string())
            .invoice_date("Fakturadatum:".to_string())
            .due_date("Förfallodatum:".to_string())
            .client_contact("Er referens:".to_string())
            .vendor_contact("Vår referens:".to_string())
            .terms("Villkor".to_string())
            .build()
    }
}

impl L18nVendorInfo {
    pub fn swedish() -> Self {
        Self::builder()
            .address("Address".to_string())
            .bank("Bank".to_string())
            .iban("IBAN".to_string())
            .bic("BIC".to_string())
            .organisation_number("Org. Nr.".to_string())
            .vat_number("Momsreg. Nr.".to_string())
            .build()
    }
}

impl L18nLineItems {
    pub fn swedish() -> Self {
        Self::builder()
            .description("Artikel".to_string())
            .when("När".to_string())
            .quantity("Antal".to_string())
            .unit_price("Enhetspris".to_string())
            .total_cost("Kostnad".to_string())
            .grand_total("Totalt:".to_string())
            .build()
    }
}

impl L18nContent {
    pub fn swedish() -> Self {
        Self::builder()
            .client_info(L18nClientInfo::swedish())
            .invoice_info(L18nInvoiceInfo::swedish())
            .vendor_info(L18nVendorInfo::swedish())
            .line_items(L18nLineItems::swedish())
            .month_names([
                "Januari".to_string(),
                "Februari".to_string(),
                "Mars".to_string(),
                "April".to_string(),
                "Maj".to_string(),
                "June".to_string(),
                "July".to_string(),
                "Augusti".to_string(),
                "September".to_string(),
                "October".to_string(),
                "November".to_string(),
                "December".to_string(),
            ])
            .build()
    }
}
