use crate::prelude::*;

pub trait FontRequiring {
    fn required_fonts(&self) -> IndexSet<FontIdentifier>;
}
