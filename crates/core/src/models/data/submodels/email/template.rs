use crate::prelude::*;

/// Template is a struct that contains the subject and body format for an email.
#[derive(Debug, Clone, Default, PartialEq, Eq, Builder, Getters, Serialize, Deserialize)]
pub struct Template {
    /// A formatting taking one argument: Invoice number, e.g. "Invoice{}".
    /// At time of composing the email, the subject will be
    /// formatted with the invoice number.
    #[getset(get = "pub")]
    subject_format: TemplatePart,
    /// A formatting taking one argument: Invoice number, e.g. "Invoice{}",
    /// and at time of composing the email, the body will be
    /// formatted with the invoice number.
    #[getset(get = "pub")]
    body_format: TemplatePart,
}

impl Template {
    pub fn materialize(&self, data: &PreparedData) -> (String, String) {
        let subject = self.subject_format.materialize(data);
        let body = self.body_format.materialize(data);
        (subject, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_materialization() {
        let template = Template {
            subject_format: TemplatePart::from("Invoice <INV_NO>"),
            body_format: TemplatePart::from("Please pay invoice <INV_NO>"),
        };
        let data = PreparedData::sample();
        let (subject, body) = template.materialize(&data);
        assert_eq!(subject, "Invoice 9876");
        assert_eq!(body, "Please pay invoice 9876");
    }
}
