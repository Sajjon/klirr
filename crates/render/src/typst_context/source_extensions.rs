use typst::syntax::{FileId, Source, VirtualPath};

use crate::prelude::*;

pub(crate) trait LoadSource: Sized {
    fn load_source_at(path: impl AsRef<Path>) -> Result<Self>;
}

pub(crate) trait InlineSource: Sized {
    fn inline(source_text: String, virtual_path: impl AsRef<Path>) -> Result<Self>;
}

impl InlineSource for Source {
    fn inline(source_text: String, virtual_path: impl AsRef<Path>) -> Result<Self> {
        // Create a new FileId for the virtual inline file ("/inline.typ").
        let file_id = FileId::new(None, VirtualPath::new(virtual_path));
        // Prepare the Typst source.
        Ok(Source::new(file_id, source_text))
    }
}

impl LoadSource for Source {
    fn load_source_at(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        // Create a new FileId for the virtual main file ("/main.typ").
        let file_id = FileId::new(None, VirtualPath::new(path));
        // Read the Typst source from the file.
        let source_text = std::fs::read_to_string(path).map_err(|e| Error::LoadSource {
            underlying: format!("{:?}", e),
        })?;
        // Prepare the Typst source.
        Ok(Source::new(file_id, source_text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_inline_source() {
        let source_text = "This is a test inline source.".to_string();
        let virtual_path = "/inline.typ";
        let source = Source::inline(source_text, virtual_path).unwrap();
        assert_eq!(
            source.id().vpath().as_rooted_path().display().to_string(),
            virtual_path
        );
        assert_eq!(source.text(), "This is a test inline source.");
    }

    #[test]
    fn test_load_source_at() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), "This is a test source.").unwrap();
        let source = Source::load_source_at(temp_file.path()).unwrap();
        assert_eq!(
            source.id().vpath().as_rooted_path().display().to_string(),
            temp_file.path().display().to_string()
        );
        assert_eq!(source.text(), "This is a test source.");
    }

    #[test]
    fn test_load_source_at_invalid_path() {
        let invalid_path = PathBuf::from("non_existent_file.typ");
        let result = Source::load_source_at(invalid_path);
        assert!(result.is_err());
    }
}
