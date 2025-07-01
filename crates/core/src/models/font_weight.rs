use crate::prelude::*;

/// A font weight is a style of a font, e.g. Regular, Bold, Italic, BoldItalic.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum FontWeight {
    /// Regular weight of the font, typically used for body text.
    Regular,

    /// Bold weight of the font, typically used for emphasis or headings.
    Bold,

    /// Italic weight of the font, typically used for emphasis or to denote titles.
    Italic,

    /// Bold Italic weight of the font, typically used for emphasis in headings or titles.
    BoldItalic,
}
