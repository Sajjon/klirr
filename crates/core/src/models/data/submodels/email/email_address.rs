use crate::prelude::*;

/// A valid email address.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::FromStr,
    derive_more::Display,
    From,
    Deref,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[display("{}", _0)]
pub struct EmailAddress(lettre::Address);
