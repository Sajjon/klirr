use crate::prelude::*;

/// A formatting taking one argument: Invoice number, e.g. "Invoice{}".
/// At time of composing the email, the subject will be
/// formatted with the invoice number.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::FromStr,
    From,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[from(String, &str)]
pub struct EmailAtomTemplate(String);
impl EmailAtomTemplate {
    const NUMBER: &str = "<INV_NO>";
    const VENDOR: &str = "<FROM_CO>";
    const CLIENT: &str = "<TO_CO>";
    const INVOICE_DATE: &str = "<INV_DATE>";

    pub fn tutorial() -> String {
        format!(
            "Placeholders: '{}', '{}', '{}', '{}'. Example: 'Invoice {} from {}' → 'Invoice 42 from Lupin et Associés'. Placeholders are case-sensitive and must include '<' and '>'.",
            Self::NUMBER,
            Self::VENDOR,
            Self::CLIENT,
            Self::INVOICE_DATE,
            Self::NUMBER,
            Self::VENDOR
        )
    }

    pub fn materialize(&self, data: &PreparedData) -> String {
        let mut raw = self.0.clone();
        raw = raw.replace(
            Self::NUMBER,
            data.information().number().to_string().as_str(),
        );
        raw = raw.replace(Self::VENDOR, data.vendor().company_name().as_str());
        raw = raw.replace(Self::CLIENT, data.client().company_name().as_str());
        raw = raw.replace(
            Self::INVOICE_DATE,
            data.information().invoice_date().to_string().as_str(),
        );

        #[cfg(debug_assertions)]
        {
            let rng = "<RNG>";
            if raw.contains(rng) {
                let rnd: u64 = rand::random();
                raw = raw.replace(rng, rnd.to_string().as_str());
            }
        }
        raw
    }
}

impl Default for EmailAtomTemplate {
    fn default() -> Self {
        Self(format!("Invoice {} from {}", Self::NUMBER, Self::VENDOR))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
        let template = EmailAtomTemplate::default();
        assert_eq!(template.0, "Invoice <INV_NO> from <FROM_CO>");
        let result = template.materialize(&PreparedData::sample());
        assert_eq!(result, "Invoice 9876 from Lupin et Associés");
    }

    #[test]
    fn test_that_tutorial_contains_all_variables() {
        let tutorial = EmailAtomTemplate::tutorial();
        assert!(tutorial.contains(EmailAtomTemplate::NUMBER));
        assert!(tutorial.contains(EmailAtomTemplate::VENDOR));
        assert!(tutorial.contains(EmailAtomTemplate::CLIENT));
        assert!(tutorial.contains(EmailAtomTemplate::INVOICE_DATE));
    }

    #[test]
    fn test_rng() {
        let template = EmailAtomTemplate::from("<RNG>");
        let result = template.materialize(&PreparedData::sample());
        let int_parsed = result.parse::<u64>();
        assert!(int_parsed.is_ok(), "Expected a number, got: {}", result);
    }
}
