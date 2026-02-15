use typst::syntax::{FileId, Source};

/// Holds all Typst sources required to build a document.
#[derive(Debug)]
pub struct Content {
    main: Source,
    modules: Vec<Source>,
}

impl Content {
    pub fn new(main: Source, modules: Vec<Source>) -> Self {
        Self { main, modules }
    }

    pub fn main(&self) -> &Source {
        &self.main
    }

    pub fn find(&self, id: FileId) -> Option<Source> {
        self.modules
            .iter()
            .find(|module| module.id() == id)
            .cloned()
    }
}
