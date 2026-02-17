#![cfg_attr(not(test), forbid(unsafe_code))]

mod render;

#[cfg(test)]
mod render_test_helpers;

pub mod prelude {
    pub use crate::render::*;

    pub use bon::Builder;
    pub use getset::Getters;
    // pub use klirr_core_invoice::prelude::*;
}
