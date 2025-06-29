use crate::prelude::*;

use typst::{Library, text::FontBook, utils::LazyHash};
use typst_kit::fonts::FontSearcher;

#[derive(Debug, Getters)]
pub struct Environment {
    #[getset(get = "pub")]
    library: LazyHash<Library>,

    #[getset(get = "pub")]
    book: LazyHash<FontBook>,

    #[getset(get = "pub")]
    fonts: Vec<typst_kit::fonts::FontSlot>,

    #[getset(get = "pub")]
    now: DateTime<Local>,
}

impl Default for Environment {
    fn default() -> Self {
        // Build the standard library (Typst definitions and styles).
        let lib = Library::builder().build();
        // Search for fonts
        let fonts_dir = dirs_next::home_dir().unwrap().join("Library/Fonts");
        let fonts_data = FontSearcher::new()
            .include_system_fonts(false)
            .search_with([fonts_dir]);

        // Get the current local date and time
        let now = Local::now();
        let book = LazyHash::new(fonts_data.book);
        let library = LazyHash::new(lib);

        Self {
            library,
            book,
            fonts: fonts_data.fonts,
            now,
        }
    }
}
