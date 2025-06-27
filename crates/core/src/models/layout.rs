use crate::prelude::*;
use derive_more::FromStr;

/// The Typst layout "Aioo" as a string.
const TYPST_LAYOUT_AIOO: &str = include_str!("../../layouts/aioo.typ");

/// A layout used for testing only.
const TYPST_LAYOUT_TEST: &str = include_str!("../../layouts/test.typ");

/// Represents different Typst layouts used to render the invoice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Default, FromStr, EnumIter)]
pub enum Layout {
    /// Originally created by [Andreas Lundblad][author], see his
    /// [blog post][blog] presenting his [Latex Template][latex].
    ///
    /// [author]: https://aioo.be/
    /// [blog]: https://aioo.be/2012/02/13/Fakturamall-Latex.html
    /// [latex]: https://aioo.be/assets/blog/invoice-template/invoice.tex
    #[default]
    Aioo,

    /// A Test layout to test if CMU font is installed.
    Test,
}

impl ToTypst for Layout {}
impl ToTypstFn for Layout {
    fn to_typst_fn(&self) -> String {
        match self {
            Self::Aioo => TYPST_LAYOUT_AIOO.to_string(),
            Self::Test => TYPST_LAYOUT_TEST.to_string(),
        }
    }
}

impl Layout {
    /// Returns all available layouts as an iterator.
    /// This can be used to iterate over all supported layouts.
    /// # Examples
    /// ```
    /// use klirr_core::prelude::*;
    /// for layout in Layout::all() {
    ///     println!("Supported layout: {}", layout);
    /// }
    /// ```
    pub fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn all_layouts_define_render_function() {
        for layout in Layout::all() {
            let typst = layout.to_typst_fn();
            assert!(
                typst.contains("#let render_invoice(data, l18n) = {"),
                "Layout {:?} does not define a render function in its Typst source: {}",
                layout,
                typst
            );
        }
    }

    #[test]
    fn test_from_str() {
        let layout: Layout = "Aioo".parse().unwrap();
        assert_eq!(layout, Layout::Aioo);

        // Test default value
        let default_layout: Layout = "Unknown".parse().unwrap_or_default();
        assert_eq!(default_layout, Layout::Aioo);
    }
}
