use clap::Args;

use crate::prelude::*;

/// CLI-parsable version of the Free enum for Clap arguments
#[derive(Args, Debug, Clone, PartialEq, Builder, Getters)]
pub struct TimeOffInput {
    /// Number of hours or days off
    #[arg(short, long)]
    #[getset(get = "pub")]
    quantity: f64,

    /// Unit of time (hours or days)
    #[arg(short, long, value_enum, default_value = "days")]
    #[getset(get = "pub")]
    unit: TimeUnitInput,
}

impl TryFrom<TimeOffInput> for TimeOff {
    type Error = klirr_core::Error;

    fn try_from(input: TimeOffInput) -> Result<Self, Self::Error> {
        let decimal = Decimal::try_from(input.quantity)?;
        let quantity = Quantity::from(decimal);
        match input.unit {
            TimeUnitInput::Hours => Ok(TimeOff::Hours(quantity)),
            TimeUnitInput::Days => Ok(TimeOff::Days(quantity)),
        }
    }
}

impl HasSample for TimeOffInput {
    fn sample() -> Self {
        Self::builder()
            .quantity(2.0)
            .unit(TimeUnitInput::Days)
            .build()
    }

    fn sample_other() -> Self {
        Self::builder()
            .quantity(16.0)
            .unit(TimeUnitInput::Hours)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = TimeOffInput;

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
