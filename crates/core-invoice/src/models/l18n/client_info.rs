use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Builder)]
pub struct L18nClientInfo {
    /// EN: "To:"
    #[getset(get = "pub")]
    to_company: String,

    /// EN: "VAT:"
    #[getset(get = "pub")]
    vat_number: String,
}

impl L18nClientInfo {
    pub fn english() -> Self {
        Self::builder()
            .to_company("To:".to_string())
            .vat_number("VAT:".to_string())
            .build()
    }
}
