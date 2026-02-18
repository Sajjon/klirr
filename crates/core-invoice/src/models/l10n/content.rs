use klirr_foundation::ToTypst;

use crate::{L10nClientInfo, L10nInvoiceInfo, L10nLineItems, L10nVendorInfo};
use bon::Builder;
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;

/// The content of the localization file, which includes
/// client information, invoice information, vendor information,
/// and line items.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, Builder)]
pub struct L10nContent {
    #[getset(get = "pub")]
    client_info: L10nClientInfo,

    #[getset(get = "pub")]
    invoice_info: L10nInvoiceInfo,

    #[getset(get = "pub")]
    vendor_info: L10nVendorInfo,

    #[getset(get = "pub")]
    line_items: L10nLineItems,

    #[getset(get = "pub")]
    month_names: [String; 12],
}
impl ToTypst for L10nContent {}
impl L10nContent {
    pub fn english() -> Self {
        Self::builder()
            .client_info(L10nClientInfo::english())
            .invoice_info(L10nInvoiceInfo::english())
            .vendor_info(L10nVendorInfo::english())
            .line_items(L10nLineItems::english())
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
