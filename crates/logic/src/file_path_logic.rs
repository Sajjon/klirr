use crate::prelude::*;

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

/// Creates a folder at `WORKSPACE_ROOT/some/relative/path` if it doesn't exist.
pub fn create_folder_relative_to_workspace(path: impl AsRef<Path>) -> Result<PathBuf> {
    let target_path = directory_relative_workspace_with_path_components(path);
    let target_folder = target_path.parent().expect("Path should have a parent");
    if !target_folder.exists() {
        fs::create_dir_all(&target_path).map_err(|e| Error::FailedToCreateOutputDirectory {
            underlying: format!("{:?}", e),
        })?;
        trace!("Created target folder: '{}'", target_folder.display());
    }
    Ok(target_path)
}
