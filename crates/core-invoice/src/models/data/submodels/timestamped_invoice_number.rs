use crate::{Date, HasSample, InvoiceNumber};
use bon::Builder;
use getset::{Getters, WithSetters};
use serde::{Deserialize, Serialize};

/// An invoice number timestamped with the period-end date it belongs to.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Builder, Getters, WithSetters)]
pub struct TimestampedInvoiceNumber {
    /// A base offset for the invoice number, e.g. `237`.
    #[builder(into)]
    #[getset(get = "pub")]
    offset: InvoiceNumber,

    /// Period-end date when the `offset` was used, e.g. `2025-05-31`.
    #[serde(alias = "period")]
    #[getset(get = "pub", set_with = "pub")]
    date: Date,
}

impl HasSample for TimestampedInvoiceNumber {
    fn sample() -> Self {
        Self::builder()
            .offset(InvoiceNumber::from(17u16))
            .date(Date::sample_other())
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .offset(InvoiceNumber::from(42u16))
            .date(Date::sample())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = TimestampedInvoiceNumber;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
