use crate::prelude::*;

pub fn get_localization(language: &Language) -> Result<L18n> {
    debug!("☑️ Reading localisation data...");
    let l18n = L18n::new(*language)?;
    debug!("✅ Read localisation data!");
    Ok(l18n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_get_localization() {
        let language = Language::EN;
        let l18n = get_localization(&language).unwrap();
        assert_eq!(*l18n.language(), language);
        assert_eq!(
            *l18n.content().invoice_info().invoice_identifier(),
            "Invoice no:"
        );
    }
}
