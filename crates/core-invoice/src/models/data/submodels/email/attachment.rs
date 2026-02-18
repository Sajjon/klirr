use crate::NamedPdf;
use derive_more::From;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum Attachment {
    Pdf(NamedPdf),
}
