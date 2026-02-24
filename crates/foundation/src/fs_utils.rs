use std::path::Path;

/// Creates a folder recursively if it does not exist.
pub fn create_folder_if_needed(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Creates the parent folder for a file path if needed.
pub fn create_folder_to_parent_of_path_if_needed(path: impl AsRef<Path>) -> std::io::Result<()> {
    let Some(parent) = path.as_ref().parent() else {
        return Ok(());
    };
    create_folder_if_needed(parent)
}

#[cfg(test)]
mod tests {

    use super::*;
    use tempfile::tempdir;
    use test_log::test;

    #[test]
    fn creates_parent_directory_without_creating_file() {
        let tmp = tempdir().unwrap();
        let file_path = tmp.path().join("a").join("b").join("file.pdf");
        create_folder_to_parent_of_path_if_needed(&file_path).unwrap();
        assert!(tmp.path().join("a").join("b").is_dir());
        assert!(!file_path.exists());
    }

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
}
