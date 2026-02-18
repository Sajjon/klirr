use crate::FontWeight;
use derive_more::Display;

/// Regular weight of Computer Modern font. For more info see [`FontIdentifier::ComputerModern`].
const FONT_COMPUTER_MODERN_REGULAR: &[u8] = include_bytes!("../../assets/cmunrm.ttf");
/// Bold weight of Computer Modern font. For more info see [`FontIdentifier::ComputerModern`].
const FONT_COMPUTER_MODERN_BOLD: &[u8] = include_bytes!("../../assets/cmunbx.ttf");

/// An identifier for a font used in typst layouts.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
#[display("{}", self.family_name())]
pub enum FontIdentifier {
    /// Font data for Computer Modern font, which is the default font used in
    /// [`Layout::Aioo`]. It is not a default system font so we include it in the
    /// klirr binary.
    ///
    /// Computer Modern is a family of typefaces designed by Donald Knuth for use in the
    /// TeX typesetting system. It is widely used in academic and scientific documents.
    /// The font is available under the SIL Open Font License, which allows for both
    /// personal and commercial use, as well as modification and redistribution.
    ComputerModern(FontWeight),
}

impl FontIdentifier {
    /// MUST match the `family_name` in the font definition, e.g. as shown in Fonts app on macOS.
    pub fn family_name(&self) -> String {
        match self {
            Self::ComputerModern(_) => "CMU Serif".to_owned(),
        }
    }

    /// The raw bytes of the font data, can be used by Typst to load the font into
    /// a Typst::Font, used by the Typst typesetting engine, this allows us to
    /// vendor the font data directly in the binary.
    pub fn font_bytes(&self) -> &'static [u8] {
        let unsupported = |weight: &str, typst_cmd: &str| {
            panic!(
                "Computer Modern {} is not supported (use of '{}' in Typst), it can easily be added if needed, create an Issue on GitHub: https://github.com/Sajjon/klirr/issues/new",
                weight, typst_cmd
            )
        };
        match self {
            Self::ComputerModern(FontWeight::Regular) => FONT_COMPUTER_MODERN_REGULAR,
            Self::ComputerModern(FontWeight::Bold) => FONT_COMPUTER_MODERN_BOLD,
            Self::ComputerModern(FontWeight::Italic) => unsupported("Italic", "emph"),
            Self::ComputerModern(FontWeight::BoldItalic) => {
                unsupported("Bold Italic", "strong[emph]")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use ttf_parser::{Face, name_id};

    #[test]
    fn test_font_identifier() {
        [FontWeight::Regular, FontWeight::Bold]
            .into_iter()
            .for_each(|weight| {
                let font = FontIdentifier::ComputerModern(weight);

                fn get_family_name(face: &Face) -> Option<String> {
                    face.names()
                        .into_iter()
                        .find(|name| name.name_id == name_id::FAMILY && name.is_unicode())
                        .and_then(|name| name.to_string())
                }
                let parsed = ttf_parser::Face::parse(font.font_bytes(), 0).unwrap();
                let family_name_of_font_parsed_from_bytes =
                    get_family_name(&parsed).unwrap_or_default();
                assert_eq!(family_name_of_font_parsed_from_bytes, "CMU Serif");
                assert_eq!(font.family_name(), family_name_of_font_parsed_from_bytes);
            });
    }

    #[test]
    #[should_panic(expected = "Computer Modern Italic is not supported")]
    fn test_italic_panics() {
        let font = FontIdentifier::ComputerModern(FontWeight::Italic);
        font.font_bytes();
    }

    #[test]
    #[should_panic(expected = "Computer Modern Bold Italic is not supported")]
    fn test_bold_italic_panics() {
        let font = FontIdentifier::ComputerModern(FontWeight::BoldItalic);
        font.font_bytes();
    }
}
