use crate::prelude::*;

/// The content of the localization file, which includes
/// client information, invoice information, vendor information,
/// and line items.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18nContent {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    client_info: L18nClientInfo,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    invoice_info: L18nInvoiceInfo,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    vendor_info: L18nVendorInfo,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    line_items: L18nLineItems,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    month_names: [String; 12],
}

impl L18nContent {
    pub fn english() -> Self {
        Self::builder()
            .client_info(L18nClientInfo::english())
            .invoice_info(L18nInvoiceInfo::english())
            .vendor_info(L18nVendorInfo::english())
            .line_items(L18nLineItems::english())
            .month_names([
                "January".to_string(),
                "February".to_string(),
                "March".to_string(),
                "April".to_string(),
                "May".to_string(),
                "June".to_string(),
                "July".to_string(),
                "August".to_string(),
                "September".to_string(),
                "October".to_string(),
                "November".to_string(),
                "December".to_string(),
            ])
            .build()
    }
}
