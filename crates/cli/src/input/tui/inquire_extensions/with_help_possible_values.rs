use inquire::{CustomType, Text};

use strum::IntoEnumIterator;

pub trait PossibleValues: IntoEnumIterator + std::fmt::Display {
    fn format_possible_values() -> String {
        format!(
            "Possible values: {}",
            Self::iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
impl<T: IntoEnumIterator + std::fmt::Display> PossibleValues for T {}

pub trait WithPossibleValues<T: PossibleValues> {
    fn with_help_possible_values(self) -> Self;
}

impl<T: PossibleValues> WithPossibleValues<T> for Text<'_> {
    fn with_help_possible_values(self) -> Self {
        // We need to use Box::leak to create a static reference
        // This is a memory leak, but acceptable for CLI help messages
        let help_message = T::format_possible_values();
        let static_help: &'static str = Box::leak(help_message.into_boxed_str());
        self.with_help_message(static_help)
    }
}

impl<T: PossibleValues + Clone> WithPossibleValues<T> for CustomType<'_, T> {
    fn with_help_possible_values(self) -> Self {
        // We need to use Box::leak to create a static reference
        // This is a memory leak, but acceptable for CLI help messages
        let help_message = T::format_possible_values();
        let static_help: &'static str = Box::leak(help_message.into_boxed_str());
        self.with_help_message(static_help)
    }
}
