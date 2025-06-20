use crate::prelude::*;
use std::{collections::HashMap, env};

/// The language used and the content of the localization file.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18n {
    /// Which language this localization file is for, e.g.
    /// "EN" for English
    #[builder(setter(into))]
    #[getset(get = "pub")]
    language: Language,

    /// The content of the localization file, which includes
    /// client information, invoice information, vendor information,
    /// and line items.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    content: L18nContent,
}

impl L18n {
    /// Tries to load a preloaded localization file for the given language.
    /// If the language is not found in the preloaded map, it returns an error.
    pub fn new(language: Language) -> Result<Self> {
        let Some(content) = L18N_MAP.get(&language) else {
            return Err(Error::L18nNotFound { language });
        };
        Ok(content.clone())
    }

    /// Tries to load the localization file for the given language.
    /// If the file is not found, it returns an error.
    /// The file is expected to be in the `input/l18n` directory
    /// relative to the `CARGO_MANIFEST_DIR`.
    /// If the file cannot be deserialized, it returns an error.
    ///
    /// # Errors
    /// Returns an error if the file is not found or if it cannot be deserialized.
    ///
    fn load_from_file(language: Language) -> Result<Self> {
        let dir = env::var("CARGO_MANIFEST_DIR").map_err(|_| Error::FileNotFound {
            path: "CARGO_MANIFEST_DIR".to_owned(),
        })?;
        let path = format!("{}/../../input/l18n/{}.ron", dir, language);
        deserialize_contents_of_ron(Path::new(&path))
    }
}

lazy_static::lazy_static! {
    static ref L18N_MAP: HashMap<Language, L18n> = {
        let mut m = HashMap::new();
        Language::all().for_each(|language| {
            m.insert(language, L18n::load_from_file(language).expect("Failed to load L18n"));
        });
        m
    };
}

#[cfg(test)]
mod tests {
    use insta::assert_ron_snapshot;

    use super::*;
    use test_log::test;

    #[test]
    fn test_l18n_english() {
        assert_ron_snapshot!(&L18n::new(Language::EN).unwrap());
    }
}
