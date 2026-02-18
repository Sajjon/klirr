use bon::Builder;
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;

/// Street address information
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Builder, Getters)]
pub struct StreetAddress {
    /// The street address line 1, of the company, e.g. `"10 West Smithfield"`.
    #[getset(get = "pub")]
    line_1: String,

    /// The street address line 2, of the company, e.g. `"C/o Other company"`.
    #[builder(default = "".to_owned())]
    #[getset(get = "pub")]
    line_2: String,
}
