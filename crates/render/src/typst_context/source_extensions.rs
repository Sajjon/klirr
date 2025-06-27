use typst::syntax::{FileId, Source, VirtualPath};

use crate::prelude::*;

pub(crate) trait InlineSource: Sized {
    fn inline(source_text: String, virtual_path: impl AsRef<Path>) -> Result<Self>;
}

impl InlineSource for Source {
    fn inline(source_text: String, virtual_path: impl AsRef<Path>) -> Result<Self> {
        // Create a new FileId for the virtual inline file.
        let file_id = FileId::new(None, VirtualPath::new(virtual_path));
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
}
