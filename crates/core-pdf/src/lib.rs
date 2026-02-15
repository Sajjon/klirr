#![cfg_attr(not(test), forbid(unsafe_code))]

mod font_identifier;
mod font_weight;
mod pdf;
mod typst;

pub use font_identifier::FontIdentifier;
pub use font_weight::FontWeight;
pub use pdf::Pdf;
pub use typst::{ToTypst, ToTypstFn};

pub mod prelude {
    pub use crate::font_identifier::*;
    pub use crate::font_weight::*;
    pub use crate::pdf::*;
    pub use crate::typst::*;
}
