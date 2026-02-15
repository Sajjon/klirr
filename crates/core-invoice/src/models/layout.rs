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
    pub fn required_fonts(&self) -> IndexSet<FontIdentifier> {
        match self {
            Self::Aioo => {
                let mut fonts = IndexSet::new();
                fonts.insert(FontIdentifier::ComputerModern(FontWeight::Regular));
                fonts.insert(FontIdentifier::ComputerModern(FontWeight::Bold));
                fonts
            }
            Self::Test => {
                let mut fonts = IndexSet::new();
                fonts.insert(FontIdentifier::ComputerModern(FontWeight::Regular));
                fonts
            }
        }
    }

    /// Returns all available layouts as an iterator.
    /// This can be used to iterate over all supported layouts.
    /// # Examples
    /// ```
    /// use klirr_core_invoice::prelude::*;
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
    use std::collections::HashSet;

    use super::*;
    use test_log::test;

    /// Returns the family names of the fonts used in the given layout.
    fn used_fonts_in_typst_file(layout: &Layout) -> HashSet<String> {
        let typst = layout.to_typst_fn();
        let mut fonts = HashSet::new();
        for line in typst.lines() {
            // we will now for each line check for patterns:
            // '    #set text(font: "CMU Serif", size: 12pt)'
            // and extract `CMU Serif` as a String
            if let Some(font) = line.split("font: ").nth(1) {
                let font_name = font
                    .split(',')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .trim_matches('"')
                    .to_string();
                if !font_name.is_empty() {
                    fonts.insert(font_name);
                }
            }
        }
        fonts
    }

    /// Returns
    fn used_font_weights_in_typst_file(layout: &Layout) -> HashSet<FontWeight> {
        // we will iterate over the lines in the Typst file and look for patterns like:
        // #strong[#emph or #emph[#strong or #emph or #strong and return the FontWeight used. Regular wont
        // be returned, as it is the default weight.
        let typst = layout.to_typst_fn();
        let mut weights = HashSet::new();
        for line in typst.lines() {
            if line.contains("#strong[#emph") || line.contains("#emph[#strong") {
                weights.insert(FontWeight::BoldItalic);
            } else if line.contains("#strong") {
                weights.insert(FontWeight::Bold);
            } else if line.contains("#emph") {
                weights.insert(FontWeight::Italic);
            }
        }
        weights
    }

    #[test]
    fn test_no_layout_uses_italic_fonts() {
        Layout::all().for_each(|layout| {
            let used_weights = used_font_weights_in_typst_file(&layout);
            assert!(
                !used_weights.contains(&FontWeight::Italic),
                "Layout {:?} uses italic fonts, not supported",
                layout
            );
            assert!(
                !used_weights.contains(&FontWeight::BoldItalic),
                "Layout {:?} uses bold italic fonts, not supported",
                layout
            );
        })
    }

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

    /// This tests helps us detect if we are writing a new layout using a font which
    /// is not defined in the `required_fonts` method.
    #[test]
    fn test_required_fonts() {
        Layout::all().for_each(|layout| {
            let all_claimed_fonts = layout
                .required_fonts()
                .into_iter()
                .map(|f| f.family_name().to_string())
                .collect::<HashSet<String>>();
            let all_identifier_fonts = used_fonts_in_typst_file(&layout);
            assert_eq!(
                all_claimed_fonts, all_identifier_fonts,
                "Layout {:?} has mismatched fonts: claimed {:?}, found {:?}",
                layout, all_claimed_fonts, all_identifier_fonts
            );
        })
    }
}
