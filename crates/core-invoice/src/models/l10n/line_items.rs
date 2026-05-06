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

    /// EN: "Subtotal:" — printed above the VAT row when VAT > 0%.
    #[getset(get = "pub")]
    subtotal: String,

    /// EN: "VAT" — label for the value-added tax row, hidden when VAT is 0%.
    #[getset(get = "pub")]
    vat: String,

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
            .subtotal("Subtotal:".to_string())
            .vat("VAT".to_string())
            .grand_total("Grand Total:".to_string())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_exposes_subtotal_and_vat_labels() {
        let sut = L10nLineItems::english();
        assert_eq!(sut.subtotal(), "Subtotal:");
        assert_eq!(sut.vat(), "VAT");
    }

    #[test]
    fn swedish_exposes_subtotal_and_vat_labels() {
        let sut = L10nLineItems::swedish();
        assert_eq!(sut.subtotal(), "Delsumma:");
        assert_eq!(sut.vat(), "Moms");
    }

    #[test]
    fn english_exposes_existing_labels() {
        let sut = L10nLineItems::english();
        assert_eq!(sut.description(), "Item");
        assert_eq!(sut.when(), "When");
        assert_eq!(sut.quantity(), "Quantity");
        assert_eq!(sut.unit_price(), "Unit price");
        assert_eq!(sut.total_cost(), "Total cost");
        assert_eq!(sut.grand_total(), "Grand Total:");
    }
}
