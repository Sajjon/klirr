use derive_more::Constructor;

use crate::prelude::*;

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
    Builder,
    Getters,
    Constructor,
)]
#[display("{year:04}-{month:02}")]
#[debug("{year:04}-{month:02}")]
pub struct YearMonthAndFortnight {
    /// e.g. 2025
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[getset(get = "pub")]
    month: Month,

    /// Either first or second half of a month
    #[getset(get = "pub")]
    half: MonthHalf,
}

impl YearMonthAndFortnight {
    fn as_year_and_month(&self) -> YearAndMonth {
        YearAndMonth::builder()
            .year(self.year)
            .month(self.month)
            .build()
    }

    fn last_day_of_half(&self) -> Day {
        match self.half {
            MonthHalf::First => Day::try_from(if self.month == Month::February {
                14
            } else {
                15
            })
            .expect("LEQ 31"),
            MonthHalf::Second => self.as_year_and_month().last_day_of_month(),
        }
    }
}

impl IsPeriod for YearMonthAndFortnight {
    fn elapsed_periods_since(&self, start: impl std::borrow::Borrow<Self>) -> u16 {
        let start = start.borrow();
        let start_ym = start.as_year_and_month();
        let end_ym = self.as_year_and_month();
        let elapsed_ym = end_ym.elapsed_months_since(start_ym);
        let elapsed_base = elapsed_ym * 2; // There are two halves per month
        elapsed_base + u16::from(*start.half())
    }

    fn to_date_end_of_period(&self) -> Date {
        Date::builder()
            .year(*self.year())
            .month(*self.month())
            .day(self.last_day_of_half())
            .build()
    }

    fn year(&self) -> &Year {
        &self.year
    }

    fn month(&self) -> &Month {
        &self.month
    }
}

impl FromStr for YearMonthAndFortnight {
    type Err = crate::Error;

    /// Parses `"YYYY-MM-2` into YearMonthAndFortnight with FortnightOfMonth being LastTwoWeeks,
    /// Parses `"YYYY-MM-1` into YearMonthAndFortnight with FortnightOfMonth being FirstTwoWeeks,
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format YearAndMonth".to_owned(),
            });
        }

        let year = Year::from_str(parts[0])?;
        let month = Month::from_str(parts[1])?;
        let half = MonthHalf::from_str(parts[2])?;

        Ok(Self::builder().year(year).month(month).half(half).build())
    }
}
