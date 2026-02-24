use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Where to save a generated output document.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OutputPath {
    /// Manually overridden path.
    AbsolutePath(PathBuf),
    /// Relative file name to be placed in a document-kind-specific output folder.
    Name(String),
}
