use derive_more::FromStr;

use crate::prelude::*;

/// A purchase order number associated with this invoice, e.g. `"PO-12345"`
/// Typically agreed upon between the vendor and client before the
/// invoice is issued.
#[derive(Clone, Debug, Display, Serialize, Deserialize, From, Deref, FromStr)]
#[from(String, &'static str)]
#[serde(transparent)]
pub struct PurchaseOrder(String);

impl HasSample for PurchaseOrder {
    fn sample() -> Self {
        Self::from("PO-12345")
    }
}
