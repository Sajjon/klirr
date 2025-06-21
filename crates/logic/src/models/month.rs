use crate::prelude::*;

/// A month of the year, e.g. 1 for January, 2 for February, etc.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Serialize, Deserialize)]
#[display("{}", self.month())]
pub enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl std::fmt::Debug for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.month())
    }
}

impl Month {
    /// Returns the month as a number, e.g. 1 for January, 2 for February, etc.
    /// This is useful for serialization and comparisons.
    /// # Examples
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// assert_eq!(Month::January.month(), &1);
    /// assert_eq!(Month::December.month(), &12);
    /// ```
    pub fn month(&self) -> &u8 {
        match self {
            Month::January => &1,
            Month::February => &2,
            Month::March => &3,
            Month::April => &4,
            Month::May => &5,
            Month::June => &6,
            Month::July => &7,
            Month::August => &8,
            Month::September => &9,
            Month::October => &10,
            Month::November => &11,
            Month::December => &12,
        }
    }
}
impl std::ops::Deref for Month {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        self.month()
    }
}

impl TryFrom<i32> for Month {
    type Error = crate::prelude::Error;

    /// Attempts to convert an integer to a `Month`.
    /// The integer must be between 1 and 12, inclusive.
    /// If the integer is outside this range, an `Error::InvalidMonth` is returned
    ///
    /// # Examples
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// let march = Month::try_from(3).unwrap();
    /// assert_eq!(march.to_string(), "3".to_owned());
    /// ```
    fn try_from(month: i32) -> Result<Self> {
        match month {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(Error::InvalidMonth {
                month,
                reason: "Month must be between 1 and 12".to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for Month {
    type Error = crate::prelude::Error;
    fn try_from(month: u8) -> Result<Self> {
        Self::try_from(month as i32)
    }
}

impl TryFrom<u32> for Month {
    type Error = crate::prelude::Error;
    fn try_from(month: u32) -> Result<Self> {
        Self::try_from(month as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use test_log::test;

    #[test]
    fn test_month_conversion() {
        assert_eq!(Month::try_from(1).unwrap(), Month::January);
        assert_eq!(Month::try_from(2).unwrap(), Month::February);
        assert_eq!(Month::try_from(10).unwrap(), Month::October);
        assert_eq!(Month::try_from(11).unwrap(), Month::November);
        assert_eq!(Month::try_from(12).unwrap(), Month::December);
        assert!(Month::try_from(0).is_err());
        assert!(Month::try_from(13).is_err());
    }

    #[test]
    fn test_month_display() {
        assert_eq!(Month::January.to_string(), "1");
        assert_eq!(Month::December.to_string(), "12");
    }

    #[test]
    fn test_month_deref() {
        let month: &u8 = &Month::March;
        assert_eq!(*month, 3);
    }

    #[test]
    fn test_month_debug() {
        assert_debug_snapshot!(Month::April, @"4");
    }
}
