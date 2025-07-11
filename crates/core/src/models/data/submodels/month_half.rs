use crate::prelude::*;

/// Either the first or the second half of a month. For february for non leap
/// years a good name for this enum would have been "fortnight", alas, it is
/// not entirely accurate for all other months.
#[derive(
    Clone,
    Copy,
    derive_more::Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum MonthHalf {
    /// The non-greedy first half of a month, i.e. 14 days for februari (including leap year),
    /// and 15 days for the other months.
    First,
    /// The remainder of days of the month after the first half have been subtracted,
    /// 15-16 days for all months except February which is 14-15 days.
    Second,
}

impl From<MonthHalf> for u16 {
    /// Converts `MonthHalf::First` into `0` and `MonthHalf::Second` into `1`.
    fn from(half: MonthHalf) -> Self {
        match half {
            MonthHalf::First => 0,
            MonthHalf::Second => 1,
        }
    }
}

impl FromStr for MonthHalf {
    type Err = crate::Error;

    /// Parses `1` into `MonthHalf::First`, and `2` into `MonthHalf::Second`.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::First),
            "2" => Ok(Self::Second),
            _ => Err(Error::FailedToParseDate {
                underlying: "Invalid Format MonthHalf".to_owned(),
            }),
        }
    }
}
