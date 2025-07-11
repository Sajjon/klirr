use crate::prelude::*;

/// A unique number for the invoice, e.g. `90`
#[derive(
    Clone, Debug, Default, Display, Serialize, Deserialize, PartialEq, Eq, Hash, From, Deref,
)]
#[serde(transparent)]
pub struct InvoiceNumber(u16);

impl std::str::FromStr for InvoiceNumber {
    type Err = crate::prelude::Error;

    /// Parses a string into an `InvoiceNumber`.
    /// Returns an error if the string is not a valid number or is out of range.
    /// # Errors
    /// Returns an `Error::InvalidInvoiceNumberString` if the string cannot be
    /// parsed into a valid `u16`.
    ///
    /// # Examples
    /// ```
    /// use klirr_core::prelude::*;
    /// let invoice_number = InvoiceNumber::from_str("1234").unwrap();
    /// assert_eq!(*invoice_number, 1234);
    /// ```
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
    fn sample_other() -> Self {
        Self::from(1234)
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

    #[test]
    fn test_from_str_valid() {
        let invoice_number = InvoiceNumber::from_str("1234").unwrap();
        assert_eq!(*invoice_number, 1234);
    }

    #[test]
    fn test_from_str_invalid() {
        let result = InvoiceNumber::from_str("invalid");
        assert!(
            result.is_err(),
            "Expected error for invalid string, got: {:?}",
            result
        );
    }
}
