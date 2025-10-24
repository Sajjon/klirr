#![cfg_attr(not(test), forbid(unsafe_code))]

mod error;
mod module;
mod render;
mod typst_context;

pub use error::{Error, Result};
pub use module::{DocumentPlan, InlineModule};
pub use render::render_document;

pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::module::{DocumentPlan, InlineModule};
    pub use crate::render::render_document;
}
