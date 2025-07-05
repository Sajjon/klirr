use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum Attachment {
    Pdf(NamedPdf),
}
