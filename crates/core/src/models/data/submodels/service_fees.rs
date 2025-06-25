use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TypedBuilder, Getters, WithSetters)]
pub struct ServiceFees {
    /// Description of the consulting service, e.g. `"Agreed Consulting Fees"`
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    name: String,
    /// The cost per item
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    unit_price: UnitPrice,
}

impl ServiceFees {
    pub fn sample() -> Self {
        Self::builder()
            .name("Agreed Consulting Fees".to_string())
            .unit_price(UnitPrice::from(350.0))
            .build()
    }
}
