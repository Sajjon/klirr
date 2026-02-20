use crate::{HasSample, InvoiceNumber, IsPeriod};
use bon::Builder;
use getset::{Getters, WithSetters};
use serde::{Deserialize, Serialize};

/// An invoice number timestamp with year and month, e.g. `(237, 2025-05)`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Builder, Getters, WithSetters)]
pub struct TimestampedInvoiceNumber<Period: IsPeriod> {
    /// A base offset for the invoice number, e.g. `237`.
    #[builder(into)]
    #[getset(get = "pub")]
    offset: InvoiceNumber,

    /// The month and year for when the `offset` was used, e.g. `2025-05`.
    #[getset(get = "pub", set_with = "pub")]
    period: Period,
}

impl<Period: IsPeriod + HasSample> HasSample for TimestampedInvoiceNumber<Period> {
    fn sample() -> Self {
        Self::builder()
            .offset(InvoiceNumber::from(17u16))
            .period(Period::sample_other())
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .offset(InvoiceNumber::from(42u16))
            .period(Period::sample())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HasSample, YearAndMonth};

    type Sut = TimestampedInvoiceNumber<YearAndMonth>;

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
