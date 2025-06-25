use crate::prelude::*;

/// Bytes represents a PDF document in memory.
#[derive(Clone, From, AsRef)]
pub struct Pdf(pub Vec<u8>);
