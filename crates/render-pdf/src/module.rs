use indexmap::IndexSet;
use klirr_core_pdf::FontIdentifier;

/// Represents an inline Typst module backed by a virtual path.
#[derive(Clone, Debug)]
pub struct InlineModule {
    virtual_path: String,
    source: String,
}

impl InlineModule {
    pub fn new(path: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            virtual_path: path.into(),
            source: source.into(),
        }
    }

    pub fn virtual_path(&self) -> &str {
        &self.virtual_path
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

/// Complete plan describing how to render a Typst document.
#[derive(Clone, Debug)]
pub struct DocumentPlan {
    fonts: IndexSet<FontIdentifier>,
    main: InlineModule,
    modules: Vec<InlineModule>,
}

impl DocumentPlan {
    pub fn new(fonts: impl IntoIterator<Item = FontIdentifier>, main: InlineModule) -> Self {
        Self {
            fonts: fonts.into_iter().collect(),
            main,
            modules: Vec::new(),
        }
    }

    pub fn with_modules(mut self, modules: impl IntoIterator<Item = InlineModule>) -> Self {
        self.modules.extend(modules);
        self
    }

    pub fn fonts(&self) -> &IndexSet<FontIdentifier> {
        &self.fonts
    }

    pub fn main(&self) -> &InlineModule {
        &self.main
    }

    pub fn modules(&self) -> &[InlineModule] {
        &self.modules
    }
}
