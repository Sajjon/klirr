use crate::prelude::*;

pub type DecryptedEmailSettings = EmailSettings<String>;
pub type EncryptedEmailSettings = EmailSettings<EncryptedAppPassword>;

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
    SerializeDisplay,
    DeserializeFromStr,
)]
pub struct EmailAtomTemplate(String);
impl EmailAtomTemplate {
    const NUMBER: &str = "<INV_NO>";
    const VENDOR: &str = "<FROM_CO>";
    const CLIENT: &str = "<TO_CO>";
    const INVOICE_DATE: &str = "<INV_DATE>";

    pub fn tutorial() -> String {
        format!(
            "Placeholder strings replaced by klirr when invoices are made, supported variables: '{}', '{}', '{}', '{}', so e.g. 'Invoice {} from {}' will become 'Invoice 42 from Lupin et Associés' when the invoice number is 42 and the vendor is Lupin et Associés.Case sensitive and must '<' & '>' are required. You can also ignore using a variable to just have a static string, .e.g 'Invoice from Lupin et Associés'.",
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
#[cfg(test)]
mod tests_atom {
    use super::*;
    #[test]
    fn test_replace() {
        let template = EmailAtomTemplate::default();
        assert_eq!(template.0, "Invoice <INV_NO> from <FROM_CO>");
        let result = template.materialize(&PreparedData::sample());
        assert_eq!(result, "Invoice 9876 from Lupin et Associés");
    }
}
impl Default for EmailAtomTemplate {
    fn default() -> Self {
        Self(format!("Invoice {} from {}", Self::NUMBER, Self::VENDOR))
    }
}

/// ProtoEmail is a struct that contains the subject and body format for an email.
#[derive(Debug, Clone, Default, PartialEq, Eq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct ProtoEmail {
    /// A formatting taking one argument: Invoice number, e.g. "Invoice{}".
    /// At time of composing the email, the subject will be
    /// formatted with the invoice number.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    subject_format: EmailAtomTemplate,
    /// A formatting taking one argument: Invoice number, e.g. "Invoice{}",
    /// and at time of composing the email, the body will be
    /// formatted with the invoice number.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    body_format: EmailAtomTemplate,
}
impl ProtoEmail {
    pub fn materialize(&self, data: &PreparedData) -> (String, String) {
        let subject = self.subject_format.materialize(data);
        let body = self.body_format.materialize(data);
        (subject, body)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct EmailSettings<AppPassword> {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    smtp_app_password: AppPassword,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    proto_email: ProtoEmail,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    reply_to: Option<EmailAccount>,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    smtp_server: SmtpServer,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    sender: EmailAccount,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    public_recipients: IndexSet<EmailAddress>,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    cc_recipients: IndexSet<EmailAddress>,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    bcc_recipients: IndexSet<EmailAddress>,
}

impl EncryptedEmailSettings {
    fn derive_and_decrypt_smtp_app_password(
        &self,
        encryption_key: EncryptionKey,
    ) -> Result<DecryptedEmailSettings> {
        let decrypted = self.smtp_app_password.decrypt(encryption_key)?;
        Ok(DecryptedEmailSettings::builder()
            .smtp_app_password(decrypted)
            .reply_to(self.reply_to.clone())
            .smtp_server(self.smtp_server.clone())
            .sender(self.sender.clone())
            .public_recipients(self.public_recipients.clone())
            .cc_recipients(self.cc_recipients.clone())
            .bcc_recipients(self.bcc_recipients.clone())
            .proto_email(self.proto_email.clone())
            .build())
    }

    pub fn decrypt_smtp_app_password(
        &self,
        encryption_password: String,
    ) -> Result<DecryptedEmailSettings> {
        let encryption_key = PbHkdfSha256::derive_key(encryption_password);
        self.derive_and_decrypt_smtp_app_password(encryption_key)
    }
}
