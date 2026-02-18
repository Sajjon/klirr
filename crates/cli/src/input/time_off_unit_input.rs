use crate::HasSample;

#[derive(clap::ValueEnum, Debug, Clone, PartialEq)]
pub enum TimeUnitInput {
    Hours,
    Days,
}

impl HasSample for TimeUnitInput {
    fn sample() -> Self {
        Self::Hours
    }

    fn sample_other() -> Self {
        Self::Days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = TimeUnitInput;

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
