use derive_more::IsVariant;

use crate::{HasSample, TimeOffInput};
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone, IsVariant, PartialEq, Default)]
pub enum TargetItems {
    /// Services mode with no time off
    #[default]
    Services,
    /// Services mode with time off specification
    ServicesOff(TimeOffInput),
    /// Expenses mode, specify expenses in `input/data/expenses.json` for the
    /// target month.
    Expenses,
}

impl HasSample for TargetItems {
    fn sample() -> Self {
        Self::Services
    }

    fn sample_other() -> Self {
        Self::ServicesOff(TimeOffInput::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = TargetItems;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
