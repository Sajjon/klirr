#![cfg_attr(not(test), forbid(unsafe_code))]

mod logic;
mod models;

pub mod prelude {

    pub use crate::logic::*;
    pub use crate::models::*;

    pub use std::{
        collections::HashMap,
        fs,
        path::{Path, PathBuf},
        str::FromStr,
    };

    pub use bon::{Builder, bon, builder};
    pub use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, Weekday};
    pub use derive_more::{AsRef, Deref, Display, From, IsVariant, TryUnwrap};
    pub use getset::{Getters, Setters, WithSetters};
    pub use indexmap::{IndexMap, IndexSet};
    pub use log::{debug, error, info, trace, warn};
    pub use rust_decimal::dec;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
    pub use serde_with::{DeserializeFromStr, SerializeDisplay};
    pub use strum::{EnumIter, IntoEnumIterator};
    pub use thiserror::Error as ThisError;
}

pub use prelude::*;
