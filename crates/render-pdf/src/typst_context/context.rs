use super::{content::Content, environment::Environment, source_extensions::InlineSource};
use crate::{
    error::{Error, Result},
    module::{DocumentPlan, InlineModule},
};
use chrono::{Datelike, FixedOffset};
use getset::Getters;
use log::trace;
use typst::{
    Library, World,
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};

/// Typst context containing the sources and environment required to build a PDF.
#[derive(Debug, Getters)]
pub struct TypstContext {
    #[getset(get = "pub")]
    content: Content,
    #[getset(get = "pub")]
    environment: Environment,
}

impl TypstContext {
    pub fn from_plan(plan: &DocumentPlan) -> Result<Self> {
        trace!("Creating TypstContext START");
        let main_source = inline_module(plan.main())?;
        let module_sources = plan
            .modules()
            .iter()
            .map(inline_module)
            .collect::<Result<Vec<_>>>()?;
        let content = Content::new(main_source, module_sources);
        let environment = Environment::new(plan.fonts().clone())?;
        trace!("Creating TypstContext END");
        Ok(Self {
            content,
            environment,
        })
    }
}

fn inline_module(module: &InlineModule) -> Result<Source> {
    Source::inline(module.source().to_owned(), module.virtual_path()).map_err(|e| {
        Error::LoadSource {
            underlying: e.to_string(),
        }
    })
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
            Ok(self.content().main().clone())
        } else if let Some(source) = self.content().find(id) {
            Ok(source)
        } else {
            panic!("Unknown typst resource requested: '{:?}'", id);
        }
    }

    fn file(&self, id: FileId) -> typst::diag::FileResult<Bytes> {
        panic!("Tried to access non-virtual file with ID: {:?}", id);
    }

    fn font(&self, index: usize) -> Option<Font> {
        if let Some(font) = self.environment().fonts().get(index).cloned() {
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
    use indexmap::IndexSet;
    use klirr_core_pdf::{FontIdentifier, FontWeight};
    use std::path::Path;
    use test_log::test;
    use typst::syntax::VirtualPath;

    fn plan() -> DocumentPlan {
        DocumentPlan::new(
            [FontIdentifier::ComputerModern(FontWeight::Regular)],
            InlineModule::new("main.typ", "main"),
        )
        .with_modules(vec![
            InlineModule::new("first.typ", "first"),
            InlineModule::new("second.typ", "second"),
        ])
    }

    fn sut() -> TypstContext {
        TypstContext::from_plan(&plan()).unwrap()
    }

    #[test]
    fn today() {
        assert!(sut().today(None).is_some())
    }

    #[test]
    #[should_panic]
    fn unknown_font_panics() {
        let context = TypstContext::from_plan(&DocumentPlan::new(
            IndexSet::new(),
            InlineModule::new("main.typ", "main"),
        ))
        .unwrap();
        let _ = context.font(1);
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
        assert!(sut.today(Some(2)).is_some());
        assert!(sut.today(Some(-2)).is_some());
    }

    #[test]
    #[should_panic]
    fn file_access_not_implemented() {
        let sut = sut();
        let _ = sut.file(FileId::new_fake(VirtualPath::new(Path::new("unknown.typ"))));
    }
}
