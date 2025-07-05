use crate::prelude::*;

/// A named sender and an email address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct EmailAccount {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    email: EmailAddress,
}

impl HasSample for EmailAccount {
    fn sample() -> Self {
        Self::builder()
            .name("Sample Sender".to_string())
            .email(EmailAddress::sample())
            .build()
    }
}
