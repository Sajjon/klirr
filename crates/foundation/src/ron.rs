use std::path::{Path, PathBuf};

use log::trace;
use serde::{Serialize, de::DeserializeOwned};

pub type Result<T, E = RonError> = std::result::Result<T, E>;

/// RON persistence errors that are domain-agnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RonError {
    FileNotFound {
        path: String,
        underlying: String,
    },
    FailedToSerialize {
        type_name: String,
        underlying: String,
    },
    FailedToWriteDataToDisk {
        underlying: String,
    },
    Deserialize {
        type_name: String,
        error: String,
    },
}

/// Returns the type name of `T`.
pub fn type_name<T>() -> String {
    std::any::type_name::<T>().to_string()
}

/// Tries to load and deserialize a RON file.
pub fn deserialize_contents_of_ron<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let ron_str = std::fs::read_to_string(path).map_err(|error| RonError::FileNotFound {
        path: path.display().to_string(),
        underlying: format!("{error:?}"),
    })?;
    deserialize_ron_str(&ron_str)
}

/// Tries to deserialize `ron_str` into `T`.
pub fn deserialize_ron_str<T: DeserializeOwned>(ron_str: &str) -> Result<T> {
    let type_name = type_name::<T>();
    trace!("☑️ Deserializing {} from RON str", type_name);
    ron::de::from_str(ron_str)
        .inspect(|_| trace!("✅ Deserialized {} from RON str", type_name))
        .map_err(|error| RonError::Deserialize {
            type_name,
            error: error.to_string(),
        })
}

/// Saves a serializable model to disk as pretty-printed RON.
pub fn save_to_disk<T: Serialize>(model: &T, path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    let ron_config = ron::ser::PrettyConfig::new().struct_names(true);
    let serialized = ron::ser::to_string_pretty(model, ron_config).map_err(|error| {
        RonError::FailedToSerialize {
            type_name: type_name::<T>(),
            underlying: format!("{error:?}"),
        }
    })?;
    std::fs::write(path, serialized).map_err(|error| RonError::FailedToWriteDataToDisk {
        underlying: format!("{error:?}"),
    })?;
    Ok(path.to_path_buf())
}

/// Builds a path `<base>/<name>.ron`.
pub fn path_to_ron_file_with_base(base_path: impl AsRef<Path>, name: &str) -> PathBuf {
    base_path.as_ref().join(format!("{name}.ron"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use tempfile::NamedTempFile;

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Example {
        name: String,
    }

    #[test]
    fn deserialize_file_not_found() {
        let result = deserialize_contents_of_ron::<Example>(PathBuf::from("/missing/file.ron"));
        assert!(matches!(result, Err(RonError::FileNotFound { .. })));
    }

    #[test]
    fn serialize_then_deserialize_roundtrip() {
        let tmp = NamedTempFile::new().unwrap();
        let expected = Example {
            name: "A".to_owned(),
        };
        save_to_disk(&expected, tmp.path()).unwrap();
        let actual: Example = deserialize_contents_of_ron(tmp.path()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn deserialize_invalid_ron() {
        let tmp = NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), "not valid ron").unwrap();
        let result = deserialize_contents_of_ron::<Example>(tmp.path());
        assert!(matches!(result, Err(RonError::Deserialize { .. })));
    }

    #[test]
    fn path_builder_adds_extension() {
        let path = path_to_ron_file_with_base("/tmp/klirr", "data");
        assert!(path.ends_with("data.ron"));
    }
}
