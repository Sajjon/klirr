use crate::prelude::*;

use typst::{
    Library,
    foundations::Bytes,
    text::{Font, FontBook},
    utils::LazyHash,
};

#[derive(Debug, Getters)]
pub struct Environment {
    #[getset(get = "pub")]
    library: LazyHash<Library>,

    #[getset(get = "pub")]
    book: LazyHash<FontBook>,

    #[getset(get = "pub")]
    fonts: Vec<Font>,

    #[getset(get = "pub")]
    now: DateTime<Local>,
}

impl Environment {
    pub fn new(fonts: IndexSet<FontIdentifier>) -> Result<Self> {
        let font_identifiers = fonts;
        // Build the standard library (Typst definitions and styles).
        let lib = Library::builder().build();

        let mut font_book = FontBook::new();
        let mut fonts = Vec::new();
        // Load the fonts into the font book and collect them into a vector.
        for font_id in font_identifiers.iter() {
            let font_bytes = font_id.font_bytes();
            let font =
                Font::new(Bytes::new(font_bytes.to_vec()), 0).ok_or(Error::FailedToLoadFont {
                    family_name: font_id.family_name(),
                })?;
            font_book.push(font.info().clone());
            fonts.push(font);
        }

        // Get the current local date and time
        let now = Local::now();
        let book = LazyHash::new(font_book);
        let library = LazyHash::new(lib);

        Ok(Self {
            library,
            book,
            fonts,
            now,
        })
    }
}
