use std::path::PathBuf;

use bon::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

/// Absolute file path and final file name of a generated document.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Builder, Getters)]
pub struct PathAndName {
    /// Absolute or resolved file path where the document should be written.
    #[getset(get = "pub")]
    path: PathBuf,

    /// File name component of `path`.
    #[getset(get = "pub")]
    name: String,
}
