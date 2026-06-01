use crate::{Date, DecryptedEmailSettings, HasSample, InvoicedItems, Language, Layout, PathBuf};
use bon::Builder;
use derive_more::Display;
use getset::{Getters, WithSetters};

/// Input validated and ready for invoice generation.
#[derive(Debug, Clone, Display, Builder, Getters, WithSetters)]
#[display("Layout: {}, Date: {}, out: {:?}, items: {}, language: {}", layout, date, maybe_output_path.as_ref().map(|d|d.display()), items, language)]
pub struct ValidInput {
    #[builder(default)]
    #[getset(get = "pub")]
    language: Language,

    /// Target period-end date used for invoice numbering and invoice date.
    #[getset(get = "pub", set_with = "pub")]
    date: Date,

    #[builder(default)]
    #[getset(get = "pub")]
    items: InvoicedItems,

    #[builder(default)]
    #[getset(get = "pub")]
    layout: Layout,

    /// Per-invoice override: when `true`, bank holidays in the target period are
    /// treated as worked, so none are deducted from billable days — even if the
    /// vendor has `off_on_bank_holidays` enabled. Defaults to `false`.
    #[builder(default)]
    #[getset(get = "pub")]
    worked_holidays: bool,

    /// When `true`, re-fetch bank holidays from the API instead of using the
    /// disk cache for this invoice (picks up corrections). Defaults to `false`.
    #[builder(default)]
    #[getset(get = "pub")]
    refresh_holidays: bool,

    #[getset(get = "pub")]
    maybe_output_path: Option<PathBuf>,

    #[getset(get = "pub")]
    email: Option<DecryptedEmailSettings>,
}

impl HasSample for ValidInput {
    fn sample() -> Self {
        Self::builder()
            .date(Date::sample())
            .items(InvoicedItems::sample())
            .maybe_output_path(PathBuf::from("invoice.pdf"))
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .date(Date::sample_other())
            .items(InvoicedItems::sample_other())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
