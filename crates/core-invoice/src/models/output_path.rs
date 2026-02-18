use crate::PathBuf;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OutputPath {
    /// Manually overridden absolute path
    AbsolutePath(PathBuf),
    /// Relative path, automatically named
    Name(String),
}
