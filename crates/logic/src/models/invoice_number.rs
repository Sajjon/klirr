use crate::prelude::*;

/// A unique number for the invoice, e.g. `90`
#[derive(Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq, From, Deref)]
#[serde(transparent)]
pub struct InvoiceNumber(u16);

impl HasSample for InvoiceNumber {
    fn sample() -> Self {
        Self::from(9876)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invoice_number_sample() {
        let sample = InvoiceNumber::sample();
        assert_eq!(*sample, 9876);
    }
}
