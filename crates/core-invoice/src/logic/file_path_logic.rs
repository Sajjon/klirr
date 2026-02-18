use crate::{Error, Path, PathBuf, Result, fs};

/// Returns the workspace root by going up from `cli` to the root.
pub fn workspace_root() -> PathBuf {
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_root
        .parent() // "../"
        .and_then(|p| p.parent()) // "../../"
        .map(PathBuf::from)
        .expect("Could not find workspace root from crate path")
}

/// Forms a path relative to the workspace root, e.g. `WORKSPACE_ROOT/some/relative/path`.
pub fn directory_relative_workspace_with_path_components(path: impl AsRef<Path>) -> PathBuf {
    let workspace = workspace_root();
    workspace.join(path.as_ref())
}

pub fn create_folder_if_needed(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| Error::FailedToCreateOutputDirectory {
            underlying: format!("{:?}", e),
        })?;
    }
    Ok(())
}

pub fn create_folder_to_parent_of_path_if_needed(path: impl AsRef<Path>) -> Result<()> {
    let Some(parent) = path.as_ref().parent() else {
        return Ok(());
    };
    create_folder_if_needed(parent)
}

/// Creates a folder at `WORKSPACE_ROOT/$relative` if it doesn't exist.
pub fn create_folder_relative_to_workspace(relative: impl AsRef<Path>) -> Result<PathBuf> {
    let path = directory_relative_workspace_with_path_components(relative);
    create_folder_to_parent_of_path_if_needed(&path)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, tempdir};
    use test_log::test;

    #[test]
    fn test_create_folder_to_parent_of_path_if_needed() {
        let tempdir = tempdir().unwrap();
        let mut base = tempdir.path().to_path_buf();
        base.push("sub0");
        let sub0 = base.clone();
        base.push("sub1");
        let sub1 = base.clone();
        create_folder_to_parent_of_path_if_needed(&sub1).unwrap();
        assert!(sub0.exists());
        assert!(sub0.is_dir());
        assert!(!sub1.exists());
        base.push("safe_to_delete.txt");
        let file = base.clone();
        create_folder_to_parent_of_path_if_needed(&file).unwrap();
        assert!(sub1.exists());
        assert!(sub1.is_dir());
        assert!(!file.exists());
    }

    #[test]
    fn test_create_relative_ws() {
        let named_file = NamedTempFile::new().unwrap();
        let file_name = named_file.path().file_name().unwrap().to_string_lossy();
        let last_two = PathBuf::from(&format!("target/{}", file_name));
        let path = create_folder_relative_to_workspace(&last_two).unwrap();
        assert!(
            path.display()
                .to_string()
                .ends_with(&last_two.display().to_string())
        )
    }
}
