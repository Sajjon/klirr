use crate::prelude::*;
use serde::de::DeserializeOwned;

/// Returns the type name of the generic type `T` as a `String`.
///
/// # Examples
/// ```
/// extern crate klirr_core;
/// use klirr_core::prelude::*;
/// let type_name = type_name::<CompanyInformation>();
/// assert!(type_name.ends_with("CompanyInformation"));
/// ```
pub fn type_name<T>() -> String {
    std::any::type_name::<T>().to_string()
}

/// Tries to load the contents of a file at the given path and deserialize it from RON format into the specified type.
///
/// # Throws
/// - `Error::FileNotFound` if the file does not exist or cannot be read
/// - `Error::Deserialize` if the contents cannot be deserialized into the specified type
pub fn deserialize_contents_of_ron<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    use std::fs;
    let path = path.as_ref();
    let ron_str = fs::read_to_string(path).map_err(|e| Error::FileNotFound {
        path: path.display().to_string(),
        underlying: format!("{:?}", e),
    })?;
    deserialize_ron_str(&ron_str)
}

pub fn deserialize_ron_str<T: DeserializeOwned>(ron_str: &str) -> Result<T> {
    use ron::de::from_str;
    let type_name = type_name::<T>();
    debug!("☑️ Deserializing {} from RON str", type_name);
    let result = from_str(ron_str)
        .inspect(|_| debug!("✅ Deserialized {} from RON str", type_name))
        .map_err(|e| Error::Deserialize {
            type_name,
            error: e.to_string(),
        })?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_type_name() {
        let type_name = type_name::<CompanyInformation>();
        assert!(type_name.ends_with("CompanyInformation"));
    }

    #[test]
    fn deserialize_contents_of_ron_fail_file_not_found() {
        let result: Result<CompanyInformation> =
            deserialize_contents_of_ron(Path::new("non_existent_file.ron"));
        assert!(result.is_err(), "Expected error, got: {:?}", result);
        if let Err(Error::FileNotFound {
            path: _,
            underlying: _,
        }) = result
        {
            // Expected error
        } else {
            panic!("Expected FileNotFound error, got: {:?}", result);
        }
    }

    #[test]
    fn deserialize_contents_of_ron_fail() {
        let tempfile = tempfile::NamedTempFile::new().unwrap();
        let path = tempfile.path();
        let result: Result<CompanyInformation> = deserialize_contents_of_ron(path);
        assert!(result.is_err(), "Expected error, got: {:?}", result);
        if let Err(Error::Deserialize { type_name, error }) = result {
            assert!(
                type_name.contains("CompanyInformation"),
                "Expected type name to contain 'CompanyInformation', got: {}",
                type_name
            );
            assert!(
                error.contains("Expected opening"),
                "Expected deserialization error, got: {}",
                error
            );
        } else {
            panic!("Expected Deserialize error, got: {:?}", result);
        }
    }

    #[test]
    fn deseralize_ron_success() {
        let tempfile = tempfile::NamedTempFile::new().unwrap();
        let path = tempfile.path();
        fs::write(
            path,
            r#"
            ServiceFees(
                name: "Agreed Consulting Service",
            	unit_price: UnitPrice(350.0)
            )"#,
        )
        .unwrap();

        let result: Result<ServiceFees> = deserialize_contents_of_ron(path);
        assert!(result.is_ok(), "Expected success, got: {:?}", result);
        let consulting_service = result.unwrap();
        assert_eq!(consulting_service.name(), "Agreed Consulting Service");
        assert_eq!(*consulting_service.unit_price(), UnitPrice::from(350.0));
    }
}
