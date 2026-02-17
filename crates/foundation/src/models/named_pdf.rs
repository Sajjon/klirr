use std::path::PathBuf;

use crate::prelude::*;

/// The outcome of generating a PDF with the produced PDF, its name, save location,
/// and the prepared data used to generate it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder, Getters)]
pub struct AbstractNamedPdf<D> {
    /// The prepared data used to generate the PDF, e.g. invoice data.
    #[getset(get = "pub")]
    prepared_data: D,

    /// The generated PDF document.
    #[getset(get = "pub")]
    pdf: Pdf,

    /// The path where the PDF is saved, e.g. "/tmp/invoice_123.pdf"
    #[getset(get = "pub")]
    saved_at: PathBuf,

    /// The name of the PDF file, e.g. "invoice_123.pdf"
    #[getset(get = "pub")]
    name: String,
}
