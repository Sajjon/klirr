use crate::{
    DecryptedEmailSettings, HasSample, InvoicedItems, Language, Layout, PathBuf, PeriodAnno,
};
use bon::Builder;
use derive_more::Display;
use getset::{Getters, WithSetters};

/// Input which has been validated and is ready for processing.
/// Controls which language to use, the month for which to generate the invoice,
/// the items to be invoiced, the layout of the invoice, and an optional output path
/// for the generated PDF file.
#[derive(Debug, Clone, Display, Builder, Getters, WithSetters)]
#[display("Layout: {}, Period: {}, out: {:?}, items: {}, language: {}", layout, period, maybe_output_path.as_ref().map(|d|d.display()), items, language)]
pub struct ValidInput {
    /// The language to use for the invoice, used on labels, headers etc.
    /// Defaults to English (`Language::EN`).
    #[builder(default)]
    #[getset(get = "pub")]
    language: Language,

    /// The period for which to generate the invoice, this affects the invoice
    /// number as well as the invoice date and due date.
    ///
    /// This period must match the configured cadence:
    /// - `Cadence::Monthly` => `PeriodAnno::YearAndMonth`
    /// - `Cadence::BiWeekly` => `PeriodAnno::YearMonthAndFortnight`
    #[getset(get = "pub", set_with = "pub")]
    period: PeriodAnno,

    /// The items to be invoiced, either services or expenses.
    #[builder(default)]
    #[getset(get = "pub")]
    items: InvoicedItems,

    /// The layout of the invoice to use
    #[builder(default)]
    #[getset(get = "pub")]
    layout: Layout,

    /// An optional override of where to save the output PDF file.
    #[getset(get = "pub")]
    maybe_output_path: Option<PathBuf>,

    /// If set, the invoice will be sent via email after generation.
    ///
    /// If set to true but email is not configured, an error will be thrown later.
    #[getset(get = "pub")]
    email: Option<DecryptedEmailSettings>,
}

impl HasSample for ValidInput {
    fn sample() -> Self {
        Self::builder()
            .period(PeriodAnno::sample())
            .items(InvoicedItems::sample())
            .maybe_output_path(PathBuf::from("invoice.pdf"))
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .period(PeriodAnno::sample_other())
            .items(InvoicedItems::sample_other())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use test_log::test;

    type Sut = ValidInput;

    #[test]
    fn valid_input_sample() {
        let sample = Sut::sample();
        assert!(sample.maybe_output_path.is_some());
    }

    #[test]
    fn valid_input_sample_other() {
        let sample = Sut::sample_other();
        assert!(sample.maybe_output_path.is_none());
    }
}
