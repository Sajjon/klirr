use crate::prelude::*;

/// An invoice number timestamp with year and month, e.g. `(237, 2025-05)`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Builder, Getters)]
pub struct TimestampedInvoiceNumber<Period: IsPeriod> {
    /// A base offset for the invoice number, e.g. `237`.
    #[getset(get = "pub")]
    offset: InvoiceNumber,

    /// The month and year for when the `offset` was used, e.g. `2025-05`.
    #[getset(get = "pub")]
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
