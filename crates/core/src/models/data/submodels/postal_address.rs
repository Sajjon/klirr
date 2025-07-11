use crate::prelude::*;

/// The postal address of a company
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Builder, Getters, WithSetters,
)]
pub struct PostalAddress {
    /// The street address of a company, e.g.
    /// ```text
    /// "10 West Smithfield"
    /// "C/o Other company"
    /// "2nd floor"
    /// ```
    #[getset(get = "pub", set_with = "pub")]
    street_address: StreetAddress,
    /// The zip code of the company, e.g. `"EC1A 1BB"`.
    #[getset(get = "pub", set_with = "pub")]
    zip: String,
    /// The country of the company, e.g. `"England"`.
    #[getset(get = "pub", set_with = "pub")]
    country: String,
    /// The city of the company, e.g. `"London"`.
    #[getset(get = "pub", set_with = "pub")]
    city: String,
}

impl HasSample for PostalAddress {
    fn sample() -> Self {
        Self::sample_client()
    }
    fn sample_other() -> Self {
        Self::sample_vendor()
    }
}

impl PostalAddress {
    pub fn sample_client() -> Self {
        Self::builder()
            .city("London".into())
            .country("England".into())
            .street_address(
                StreetAddress::builder()
                    .line_1("221B Baker Street".into())
                    .build(),
            )
            .zip("NW1 6XE".into())
            .build()
    }

    pub fn sample_vendor() -> Self {
        Self::builder()
            .city("Paris".into())
            .country("France".into())
            .street_address(
                StreetAddress::builder()
                    .line_1("5 Avenue Henri-Martin".into())
                    .line_2("Appartement 24".into())
                    .build(),
            )
            .zip("75116".into())
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
