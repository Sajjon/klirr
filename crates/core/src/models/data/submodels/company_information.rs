use crate::prelude::*;

/// Information about a company
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, TypedBuilder, Getters, WithSetters,
)]
pub struct CompanyInformation {
    /// Name of the person responsible for the invoice, e.g. `"John Doe"`.
    ///
    /// Used for "Our reference" in case of vendor, or
    /// "For attestation of", in case of client.
    #[builder(setter(into, strip_option))]
    #[getset(get = "pub", set_with = "pub")]
    contact_person: Option<String>,
    /// The unique organisation number of the company, e.g. `"123456789"`.
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    organisation_number: String,
    /// The name of the company
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    company_name: String,
    /// The postal address of the company
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    postal_address: PostalAddress,
    /// The VAT number of the company, e.g. `"GB123456789"`.
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    vat_number: String,
}

impl HasSample for CompanyInformation {
    fn sample() -> Self {
        Self::sample_client()
    }
}

impl CompanyInformation {
    pub fn sample_client() -> Self {
        Self::builder()
            .company_name("Holmes Ltd")
            .contact_person("Sherlock Holmes")
            .organisation_number("9876543-2101")
            .postal_address(PostalAddress::sample_client())
            .vat_number("GB987654321")
            .build()
    }

    pub fn sample_vendor() -> Self {
        Self::builder()
            .company_name("Lupin et Associés")
            .contact_person("Arsène Lupin")
            .organisation_number("7418529-3012")
            .postal_address(PostalAddress::sample_vendor())
            .vat_number("FR74185293012")
            .build()
    }
}
