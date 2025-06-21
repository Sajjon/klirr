use crate::prelude::*;

/// The postal address of a company
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct PostalAddress {
    /// The street address of a company, e.g.
    /// ```text
    /// "10 West Smithfield"
    /// "C/o Other company"
    /// "2nd floor"
    /// ```
    #[builder(setter(into))]
    #[getset(get = "pub")]
    street_address: StreetAddress,
    /// The zip code of the company, e.g. `"EC1A 1BB"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    zip: String,
    /// The country of the company, e.g. `"England"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    country: String,
    /// The city of the company, e.g. `"London"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    city: String,
}

impl HasSample for PostalAddress {
    fn sample() -> Self {
        Self::sample_client()
    }
}

impl PostalAddress {
    pub fn sample_client() -> Self {
        Self::builder()
            .city("London")
            .country("England")
            .street_address(StreetAddress::builder().line_1("221B Baker Street").build())
            .zip("NW1 6XE")
            .build()
    }

    pub fn sample_vendor() -> Self {
        Self::builder()
            .city("Stockholm")
            .country("Sweden")
            .street_address(
                StreetAddress::builder()
                    .line_1("Storgatan 45")
                    .line_2("4 tr")
                    .build(),
            )
            .zip("114 32")
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use test_log::test;

    #[test]
    fn test_debug() {
        assert_debug_snapshot!(PostalAddress::sample(), @r###"
        PostalAddress {
            street_address: StreetAddress {
                line_1: "221B Baker Street",
                line_2: "",
            },
            zip: "NW1 6XE",
            country: "England",
            city: "London",
        }
        "###);
    }
}
