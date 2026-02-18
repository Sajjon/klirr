use bon::Builder;
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;

/// Localization for line items in the invoice, used in the
/// table of items being billed for.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, Builder)]
pub struct L10nLineItems {
    /// EN: "Item"
    #[getset(get = "pub")]
    description: String,

    /// EN: "When"
    #[getset(get = "pub")]
    when: String,

    /// EN: "Quantity"
    #[getset(get = "pub")]
    quantity: String,

    /// EN: "Unit price"
    #[getset(get = "pub")]
    unit_price: String,

    /// EN: "Total cost"
    #[getset(get = "pub")]
    total_cost: String,

    /// EN: "Grand Total:"
    #[getset(get = "pub")]
    grand_total: String,
}

impl L10nLineItems {
    pub fn english() -> Self {
        Self::builder()
            .description("Item".to_string())
            .when("When".to_string())
            .quantity("Quantity".to_string())
            .unit_price("Unit price".to_string())
            .total_cost("Total cost".to_string())
            .grand_total("Grand Total:".to_string())
            .build()
    }
}
