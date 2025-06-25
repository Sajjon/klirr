use crate::prelude::*;

/// The day of the month, e.g. 1 for the first day, 31 for the last day of a month.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize, Deref)]
pub struct Day(u8);

impl HasSample for Day {
    fn sample() -> Self {
        Self(1)
    }
}

impl std::str::FromStr for Day {
    type Err = crate::prelude::Error;

    /// Parses a day from a string, e.g. "1" for the first day, "31" for the last day of a month.
    /// # Errors
    /// Returns an error if the string is not a valid day or if it is out of range.
    /// # Examples
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let day: Day = "15".parse().unwrap();
    /// assert_eq!(*day, 15);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = s.parse::<i32>().map_err(|_| Error::InvalidDayFromString {
            invalid_string: s.to_string(),
            reason: "Invalid day format".to_string(),
        })?;
        Self::try_from(day)
    }
}

impl TryFrom<i32> for Day {
    type Error = crate::prelude::Error;
    fn try_from(day: i32) -> Result<Self> {
        if !(1..=31).contains(&day) {
            return Err(Error::InvalidDay {
                day,
                reason: "Day must be between 1 and 31".to_string(),
            });
        }
        Ok(Self(day as u8))
    }
}

impl TryFrom<u8> for Day {
    type Error = crate::prelude::Error;
    fn try_from(day: u8) -> Result<Self> {
        Self::try_from(day as i32)
    }
}

impl TryFrom<u32> for Day {
    type Error = crate::prelude::Error;
    fn try_from(day: u32) -> Result<Self> {
        Self::try_from(day as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_day_conversion() {
        assert_eq!(Day::try_from(1).unwrap(), Day(1));
        assert_eq!(Day::try_from(15).unwrap(), Day(15));
        assert_eq!(Day::try_from(31).unwrap(), Day(31));
        assert!(Day::try_from(0).is_err());
        assert!(Day::try_from(32).is_err());
    }

    #[test]
    fn test_day_from_str() {
        let day: Day = "15".parse().unwrap();
        assert_eq!(day, Day(15));
    }

    #[test]
    fn test_day_from_invalid_all_reasons() {
        let invalid_strings = [
            "0",    // Invalid day (0)
            "32",   // Invalid day (greater than 31)
            "-1",   // Invalid day (negative)
            "abc",  // Invalid format (not a number)
            "15.5", // Invalid format (decimal)
        ];

        for &s in &invalid_strings {
            assert!(Day::from_str(s).is_err(), "Expected error for input: {}", s);
        }
    }
}
