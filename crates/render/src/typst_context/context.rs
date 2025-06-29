use crate::prelude::*;

use chrono::FixedOffset;
use typst::{
    Library, World,
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};

/// A typst context that contains the necessary
/// sources and environment to render an invoice.
#[derive(Debug, Getters)]
pub struct TypstContext {
    /// The typst source files used to render the invoices layout and data.
    #[getset(get = "pub")]
    content: Content,

    /// The environment containing the library, font book, and current time.
    #[getset(get = "pub")]
    environment: Environment,
}

impl TypstContext {
    fn new(main: Source, layout: Source, l18n: Source, data: Source) -> Self {
        trace!("Creating TypstContext START");
        let content = Content::builder()
            .main(main)
            .layout(layout)
            .data(data)
            .l18n(l18n)
            .build();
        let environment = Environment::default();

        trace!("Creating TypstContext END");
        Self {
            content,
            environment,
        }
    }

    pub fn with_inline(
        main_inline: String,
        layout_inline: String,
        l18n_inline: String,
        data_inline: String,
    ) -> Result<Self> {
        Ok(Self::new(
            Source::inline(main_inline, Path::new(TYPST_VIRTUAL_NAME_MAIN))?,
            Source::inline(layout_inline, Path::new(TYPST_VIRTUAL_NAME_LAYOUT))?,
            Source::inline(l18n_inline, Path::new(TYPST_VIRTUAL_NAME_L18N))?,
            Source::inline(data_inline, Path::new(TYPST_VIRTUAL_NAME_DATA))?,
        ))
    }
}

impl World for TypstContext {
    fn library(&self) -> &LazyHash<Library> {
        self.environment().library()
    }

    fn book(&self) -> &LazyHash<FontBook> {
        self.environment().book()
    }

    fn main(&self) -> FileId {
        self.content().main().id()
    }

    fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
        if id == self.content().main().id() {
            let source = self.content().main().clone();
            Ok(source)
        } else if id == self.content().layout().id() {
            let source = self.content().layout().clone();
            Ok(source)
        } else if id == self.content.l18n().id() {
            let source = self.content().l18n().clone();
            Ok(source)
        } else if id == self.content.data().id() {
            let source = self.content().data().clone();
            Ok(source)
        } else {
            panic!("Unknown typst resource requested: '{:?}'", id);
        }
    }

    fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
        panic!("File access not implemented in this minimal example")
    }

    fn font(&self, index: usize) -> Option<Font> {
        let Some(font_slot) = self.environment().fonts().get(index) else {
            panic!("Font Slot not found at index: {}", index);
        };

        if let Some(font) = font_slot.get() {
            Some(font)
        } else {
            panic!("Font not found at index: {}", index);
        }
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let now = self.environment().now();
        let with_offset = match offset {
            None => now.with_timezone(now.offset()).fixed_offset(),
            Some(hours) => {
                let seconds = i32::try_from(hours).ok()?.checked_mul(3600)?;
                let fixed = FixedOffset::east_opt(seconds)?;
                now.with_timezone(&fixed)
            }
        };

        Datetime::from_ymd(
            with_offset.year(),
            with_offset.month().try_into().ok()?,
            with_offset.day().try_into().ok()?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use typst::syntax::VirtualPath;

    fn sut() -> TypstContext {
        TypstContext::new(
            Source::detached("main"),
            Source::detached("layout"),
            Source::detached("l18n"),
            Source::detached("data"),
        )
    }

    #[test]
    fn today() {
        let sut = sut();
        let today = sut.today(None);
        assert!(today.is_some())
    }

    #[test]
    #[should_panic]
    fn unknown_font_panics() {
        let sut = sut();
        let _ = sut.font(9999999999);
    }

    #[test]
    #[should_panic]
    fn unknown_typst_resource_panics() {
        let sut = sut();
        let _ = sut.source(FileId::new_fake(VirtualPath::new(Path::new("unknown.typ"))));
    }

    #[test]
    fn today_with_offset() {
        let sut = sut();
        let today = sut.today(Some(2));
        assert!(today.is_some());
        let today = sut.today(Some(-2));
        assert!(today.is_some());
    }

    #[test]
    #[should_panic]
    fn file_access_not_implemented() {
        let sut = sut();
        let _ = sut.file(FileId::new_fake(VirtualPath::new(Path::new("unknown.typ"))));
    }
}
