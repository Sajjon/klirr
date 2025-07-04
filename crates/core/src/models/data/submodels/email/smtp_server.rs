use crate::prelude::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    AsRef,
    derive_more::Display,
    derive_more::FromStr,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct SmtpServer(String);

impl SmtpServer {
    pub fn gmail() -> Self {
        SmtpServer("smtp.gmail.com".to_owned())
    }
}
impl Default for SmtpServer {
    fn default() -> Self {
        SmtpServer::gmail()
    }
}
