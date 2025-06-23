use crate::prelude::*;

/// Localization for vendor information in the invoice,
/// such as bank details and organization information.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18nVendorInfo {
    /// EN: "Address"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    address: String,

    /// EN: "Bank"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    bank: String,

    /// EN: "IBAN"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    iban: String,

    /// EN: "BIC"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    bic: String,

    /// EN: "Org. No."
    #[builder(setter(into))]
    #[getset(get = "pub")]
    organisation_number: String,

    /// EN: "VAT No."
    #[builder(setter(into))]
    #[getset(get = "pub")]
    vat_number: String,
}

impl L18nVendorInfo {
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
