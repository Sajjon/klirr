use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OutputPath {
    /// Manually overridden absolute path
    AbsolutePath(PathBuf),
    /// Relative path, automatically named
    Name(String),
}
