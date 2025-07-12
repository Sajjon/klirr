#![cfg_attr(not(test), forbid(unsafe_code))]

mod render;
mod typst_context;

#[cfg(test)]
mod render_test_helpers;

pub mod prelude {
    pub use crate::render::*;
    pub(crate) use crate::typst_context::*;

    pub use bon::Builder;
    pub use getset::Getters;
    pub use klirr_core::prelude::*;
}
