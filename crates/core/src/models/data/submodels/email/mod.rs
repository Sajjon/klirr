mod attachment;
#[allow(clippy::module_inception)]
mod email;
mod email_account;
mod email_address;
mod email_atom_template;
mod email_credentials;
mod email_settings;
mod proto_email;
mod smtp_server;

pub use attachment::*;
pub use email::*;
pub use email_account::*;
pub use email_address::*;
pub use email_atom_template::*;
pub use email_credentials::*;
pub use email_settings::*;
pub use proto_email::*;
pub use smtp_server::*;
