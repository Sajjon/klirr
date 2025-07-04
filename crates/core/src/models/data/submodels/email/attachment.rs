use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Attachment {
    Pdf(NamedPdf),
}
