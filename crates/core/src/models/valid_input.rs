use crate::prelude::*;

/// Input which has been validated and is ready for processing.
/// Controls which language to use, the month for which to generate the invoice,
/// the items to be invoiced, the layout of the invoice, and an optional output path
/// for the generated PDF file.
#[derive(Debug, Clone, Display, TypedBuilder, Getters)]
#[display("Layout: {}, Month: {}, out: {:?}, items: {}, language: {}", layout, month, maybe_output_path.as_ref().map(|d|d.display()), items, language)]
pub struct ValidInput {
    /// The language to use for the invoice, used on labels, headers etc.
    /// Defaults to English (`Language::EN`).
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    language: Language,

    /// The month for which to generate the invoice, this affects the invoice
    /// number as well as the invoice date and due date.
    #[getset(get = "pub")]
    month: YearAndMonth,

    /// The items to be invoiced, either services or expenses.
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    items: InvoicedItems,

    /// The layout of the invoice to use
    #[builder(setter(into), default)]
    #[getset(get = "pub")]
    layout: Layout,

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
