#![cfg_attr(not(test), forbid(unsafe_code))]

#[cfg(test)]
mod compare_images;
mod error;
mod module;
mod render;
mod typst_context;

#[cfg(test)]
mod render_test_helpers;

pub use error::{Error, Result};
pub use klirr_foundation::{FontIdentifier, FontRequiring, FontWeight, Pdf, ToTypst, ToTypstFn};
pub use module::{DocumentPlan, InlineModule};
pub use render::*;
