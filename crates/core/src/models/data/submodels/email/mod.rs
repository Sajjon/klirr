mod attachment;
#[allow(clippy::module_inception)]
mod email;
mod email_account;
mod email_address;
mod email_credentials;
mod email_settings;
mod lettre_bridge;
mod smtp_server;
mod template;
mod template_part;

pub use attachment::*;
pub use email::*;
pub use email_account::*;
pub use email_address::*;
pub use email_credentials::*;
pub use email_settings::*;
pub use lettre_bridge::*;
pub use smtp_server::*;
pub use template::*;
pub use template_part::*;
