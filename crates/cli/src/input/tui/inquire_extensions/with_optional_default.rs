use inquire::{CustomType, Text};

pub trait WithOptionalDefault<'o, T> {
    fn with_optional_default(self, default: &'o Option<T>) -> Self;
}

impl<'a, 'o: 'a, T: AsRef<str>> WithOptionalDefault<'o, T> for Text<'a> {
    fn with_optional_default(self, default: &'o Option<T>) -> Self {
        match default {
            Some(value) => self.with_default(value.as_ref()),
            None => self,
        }
    }
}
impl<'a, 'o: 'a, T: Clone> WithOptionalDefault<'o, T> for CustomType<'a, T> {
    fn with_optional_default(self, default: &'o Option<T>) -> Self {
        match default {
            Some(value) => self.with_default(value.clone()),
            None => self,
        }
    }
}

pub trait WithOptionalRefDefault<'o, T> {
    fn with_optional_ref_default(self, default: Option<&'o T>) -> Self;
}

impl<'a, 'o: 'a, T: AsRef<str>> WithOptionalRefDefault<'o, T> for Text<'a> {
    fn with_optional_ref_default(self, default: Option<&'o T>) -> Self {
        match default {
            Some(value) => self.with_default(value.as_ref()),
            None => self,
        }
    }
}
impl<'a, 'o: 'a, T: Clone> WithOptionalRefDefault<'o, T> for CustomType<'a, T> {
    fn with_optional_ref_default(self, default: Option<&'o T>) -> Self {
        match default {
            Some(value) => self.with_default(value.clone()),
            None => self,
        }
    }
}
