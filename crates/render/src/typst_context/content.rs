use crate::prelude::*;
use typst::syntax::Source;

/// The typst source files used to render the invoices layout and data.
#[derive(Debug, Getters, Builder)]
pub struct Content {
    /// The main typst file that contains calls the render function of layout, with the data and l18n files.
    #[getset(get = "pub")]
    main: Source,

    /// The static invoice typst file with the layout
    #[getset(get = "pub")]
    layout: Source,

    /// The localization file for the invoice, used for
    /// translations of all static text elements.
    #[getset(get = "pub")]
    l18n: Source,

    /// The data source for the invoice, which contains
    /// the dynamic data to be rendered in the invoice,
    /// such as vendor and client information, items, and totals.
    #[getset(get = "pub")]
    data: Source,
}
