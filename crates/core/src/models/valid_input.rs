use crate::prelude::*;

#[derive(Debug, Clone, Display, TypedBuilder, Getters)]
#[display("Month: {}, out: {:?}, items: {}, language: {}", month, maybe_output_path.as_ref().map(|d|d.display()), items, language)]
pub struct ValidInput {
    #[builder(setter(into), default = Language::EN)]
    #[getset(get = "pub")]
    language: Language,

    #[getset(get = "pub")]
    month: YearAndMonth,

    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    items: InvoicedItems,

    /// An optional override of where to save the output PDF file.
    #[builder(default, setter(into))]
    #[getset(get = "pub")]
    maybe_output_path: Option<PathBuf>,
}

impl HasSample for ValidInput {
    fn sample() -> Self {
        Self::builder()
            .month(YearAndMonth::current())
            .items(InvoicedItems::sample())
            .maybe_output_path(Some(PathBuf::from("invoice.pdf")))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn valid_input_sample() {
        let sample = ValidInput::sample();
        assert!(sample.maybe_output_path.is_some());
    }
}
