use crate::FontIdentifier;
use indexmap::IndexSet;

pub trait FontRequiring {
    fn required_fonts(&self) -> IndexSet<FontIdentifier>;
}
