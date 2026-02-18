use bon::Builder;
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Builder)]
pub struct L10nClientInfo {
    /// EN: "To:"
    #[getset(get = "pub")]
    to_company: String,

    /// EN: "VAT:"
    #[getset(get = "pub")]
    vat_number: String,
}

impl L10nClientInfo {
    pub fn english() -> Self {
        Self::builder()
            .to_company("To:".to_string())
            .vat_number("VAT:".to_string())
            .build()
    }
}
