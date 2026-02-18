#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod compare_images;
mod error;
mod module;
mod render;
mod typst_context;

pub use error::{Error, Result};
pub use klirr_foundation::{FontIdentifier, FontRequiring, FontWeight, Pdf, ToTypst, ToTypstFn};
pub use module::{DocumentPlan, InlineModule};
pub use render::render_document;
