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
    EnumIter,
)]
pub enum MonthHalf {
    /// The non-greedy first half of a month, i.e. 14 days for februari (including leap year),
    /// and 15 days for the other months.
    #[display("first-half")]
    First,
    /// The remainder of days of the month after the first half have been subtracted,
    /// 15-16 days for all months except February which is 14-15 days.
    #[display("second-half")]
    Second,
}

impl From<MonthHalf> for i16 {
    /// Converts `MonthHalf::First` into `1` and `MonthHalf::Second` into `2`.
    fn from(half: MonthHalf) -> Self {
        match half {
            MonthHalf::First => 1,
            MonthHalf::Second => 2,
        }
    }
}

impl From<NaiveDate> for MonthHalf {
    fn from(value: NaiveDate) -> Self {
        let date = Date::from(value);
        Self::from(date)
    }
}

impl From<Date> for MonthHalf {
    fn from(value: Date) -> Self {
        let day = **value.day();
        if value.month().is_february() {
            return if day <= 14 { Self::First } else { Self::Second };
        }
        if day <= 15 { Self::First } else { Self::Second }
    }
}

impl FromStr for MonthHalf {
    type Err = crate::Error;

    /// Parses `1` into `MonthHalf::First`, and `2` into `MonthHalf::Second`.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1" => Ok(Self::First),
            "first-half" => Ok(Self::First),
            "first" => Ok(Self::First),
            "2" => Ok(Self::Second),
            "second-half" => Ok(Self::Second),
            "second" => Ok(Self::Second),
            _ => Err(Error::FailedToParseDate {
                underlying: "Invalid Format MonthHalf".to_owned(),
            }),
        }
    }
}

impl MonthHalf {
    pub fn now() -> Self {
        let today = chrono::Local::now().date_naive();
        let date = Date::from(today);
        MonthHalf::from(date)
    }
}
impl HasSample for MonthHalf {
    fn sample() -> Self {
        Self::First
    }

    fn sample_other() -> Self {
        Self::Second
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    type Sut = MonthHalf;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn display_sample() {
        assert_snapshot!(Sut::sample());
    }

    #[test]
    fn display_sample_other() {
        assert_snapshot!(Sut::sample_other());
    }

    #[test]
    fn test_from_str_all_valid() {
        assert_eq!(Sut::from_str("1").unwrap(), Sut::First);
        assert_eq!(Sut::from_str("first-half").unwrap(), Sut::First);
        assert_eq!(Sut::from_str("first").unwrap(), Sut::First);
        assert_eq!(Sut::from_str("2").unwrap(), Sut::Second);
        assert_eq!(Sut::from_str("second-half").unwrap(), Sut::Second);
        assert_eq!(Sut::from_str("second").unwrap(), Sut::Second);
    }

    #[test]
    fn test_into_i16() {
        assert_eq!(i16::from(Sut::First), 1);
        assert_eq!(i16::from(Sut::Second), 2);
    }
}
