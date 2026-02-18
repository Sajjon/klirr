use clap::Args;

use crate::PeriodAnno;
use getset::Getters;

/// Record a new period off for the specified period.
#[derive(Debug, Args, Getters, PartialEq)]
pub struct PeriodOffInput {
    /// The period to be added if not already present in the data directory.
    #[arg(
        long,
        short = 'p',
        default_value = None,
        help = "The period for which you wanna record a period off, e.g. `2025-05`."
    )]
    #[getset(get = "pub")]
    period: PeriodAnno,
}
