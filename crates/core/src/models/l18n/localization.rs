use crate::prelude::*;
use std::collections::HashMap;

/// The language used and the content of the localization file.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, Builder)]
pub struct L18n {
    /// Which language this localization file is for, e.g.
    /// "EN" for English
    #[getset(get = "pub")]
    language: Language,

    /// The content of the localization file, which includes
    /// client information, invoice information, vendor information,
    /// and line items.
    #[getset(get = "pub")]
    content: L18nContent,
}

impl L18n {
    pub fn english() -> Self {
        Self::builder()
            .language(Language::EN)
            .content(L18nContent::english())
            .build()
    }
}

impl L18n {
    /// Tries to load a preloaded localization file for the given language.
    /// If the language is not found in the preloaded map, it returns an error.
    pub fn new(language: Language) -> Result<Self> {
        let content = L18N_MAP
            .get(&language)
            .expect("Every language should be preloaded");
        Ok(content.clone())
    }
}

lazy_static::lazy_static! {
    static ref L18N_MAP: HashMap<Language, L18n> = {
        let mut m = HashMap::new();
        let mut add = |localization: L18n| {
            m.contains_key(localization.language())
                .then(|| panic!("Localization for {:?} already exists", localization.language()));
            m.insert(localization.language, localization);
        };
        add(L18n::english());
        add(L18n::swedish());
        Language::all()
            .for_each(|lang| {
                if !m.contains_key(&lang) {
                    panic!("No localization found for {:?}", lang);
                }
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

    #[test]
    fn test_l18n_swedish() {
        assert_ron_snapshot!(&L18n::new(Language::SV).unwrap());
    }
}
