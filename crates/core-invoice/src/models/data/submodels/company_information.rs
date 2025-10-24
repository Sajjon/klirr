use crate::prelude::*;

/// Information about a company
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Builder, Getters, WithSetters,
)]
pub struct CompanyInformation {
    /// Name of the person responsible for the invoice, e.g. `"John Doe"`.
    ///
    /// Used for "Our reference" in case of vendor, or
    /// "For attestation of", in case of client.
    #[getset(get = "pub", set_with = "pub")]
    contact_person: Option<String>,
    /// The unique organisation number of the company, e.g. `"123456789"`.
    #[getset(get = "pub", set_with = "pub")]
    organisation_number: String,
    /// The name of the company
    #[getset(get = "pub", set_with = "pub")]
    company_name: String,
    /// The postal address of the company
    #[getset(get = "pub", set_with = "pub")]
    postal_address: PostalAddress,
    /// The VAT number of the company, e.g. `"GB123456789"`.
    #[getset(get = "pub", set_with = "pub")]
    vat_number: String,
}

impl HasSample for CompanyInformation {
    fn sample() -> Self {
        Self::sample_client()
    }

    fn sample_other() -> Self {
        Self::sample_vendor()
    }
}

impl CompanyInformation {
    pub fn sample_client() -> Self {
        Self::builder()
            .company_name("Holmes Ltd".into())
            .contact_person("Sherlock Holmes".into())
            .organisation_number("9876543-2101".into())
            .postal_address(PostalAddress::sample_client())
            .vat_number("GB987654321".into())
            .build()
    }

    pub fn sample_vendor() -> Self {
        Self::builder()
            .company_name("Lupin et Associés".into())
            .contact_person("Arsène Lupin".into())
            .organisation_number("7418529-3012".into())
            .postal_address(PostalAddress::sample_vendor())
            .vat_number("FR74185293012".into())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = CompanyInformation;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
