use crate::{L10n, Language, Result};
use log::debug;

pub fn get_localization(language: &Language) -> Result<L10n> {
    debug!("☑️ Reading localisation data...");
    let l10n = L10n::new(*language)?;
    debug!("✅ Read localisation data!");
    Ok(l10n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_get_localization() {
        let language = Language::EN;
        let l10n = get_localization(&language).unwrap();
        assert_eq!(*l10n.language(), language);
        assert_eq!(
            *l10n.content().invoice_info().invoice_identifier(),
            "Invoice no:"
        );
    }
}
