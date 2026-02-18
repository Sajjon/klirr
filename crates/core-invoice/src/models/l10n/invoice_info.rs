use bon::Builder;
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;

/// Localization for invoice information, such as purchase order,
/// invoice number, dates, and terms.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, Builder)]
pub struct L10nInvoiceInfo {
    /// EN: "Purchase order:"
    #[getset(get = "pub")]
    purchase_order: String,

    /// EN: "Invoice no:"
    #[getset(get = "pub")]
    invoice_identifier: String,

    /// EN: "Invoice date:"
    #[getset(get = "pub")]
    invoice_date: String,

    /// EN: "Due date:"
    #[getset(get = "pub")]
    due_date: String,

    /// EN: "For the attention of:"
    #[getset(get = "pub")]
    client_contact: String,

    /// EN: "Our reference:"
    #[getset(get = "pub")]
    vendor_contact: String,

    /// EN: "Terms"
    #[getset(get = "pub")]
    terms: String,
}

impl L10nInvoiceInfo {
    pub fn english() -> Self {
        Self::builder()
            .purchase_order("Purchase order:".to_string())
            .invoice_identifier("Invoice no:".to_string())
            .invoice_date("Invoice date:".to_string())
            .due_date("Due date:".to_string())
            .client_contact("For the attention of:".to_string())
            .vendor_contact("Our reference:".to_string())
            .terms("Terms:".to_string())
            .build()
    }
}
