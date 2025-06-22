use crate::prelude::*;

/// A unique number for the invoice, e.g. `90`
#[derive(Clone, Debug, Default, Display, Serialize, Deserialize, PartialEq, Eq, From, Deref)]
#[serde(transparent)]
pub struct InvoiceNumber(u16);

impl std::str::FromStr for InvoiceNumber {
    type Err = crate::prelude::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.parse::<u16>()
            .map(InvoiceNumber)
            .map_err(|_| Error::InvalidInvoiceNumberString {
                invalid_string: s.to_owned(),
            })
    }
}

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

    #[test]
    fn test_invoice_number_default_is_zero() {
        let default = InvoiceNumber::default();
        assert_eq!(*default, 0);
    }
}
