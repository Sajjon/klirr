use bon::Builder;
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;

/// Localization for vendor information in the invoice,
/// such as bank details and organization information.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, Builder)]
pub struct L10nVendorInfo {
    /// EN: "Address"
    #[getset(get = "pub")]
    address: String,

    /// EN: "Bank"
    #[getset(get = "pub")]
    bank: String,

    /// EN: "IBAN"
    #[getset(get = "pub")]
    iban: String,

    /// EN: "BIC"
    #[getset(get = "pub")]
    bic: String,

    /// EN: "Org. No."
    #[getset(get = "pub")]
    organisation_number: String,

    /// EN: "VAT No."
    #[getset(get = "pub")]
    vat_number: String,
}

impl L10nVendorInfo {
    pub fn english() -> Self {
        Self::builder()
            .address("Address".to_string())
            .bank("Bank".to_string())
            .iban("IBAN".to_string())
            .bic("BIC".to_string())
            .organisation_number("Org. No.".to_string())
            .vat_number("VAT No.".to_string())
            .build()
    }
}
