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
        let elapsed = end_ym.elapsed_months_since(start_ym);
        let elapsed_ym = (elapsed * 2) as i16; // two halves per month
        let start_half = i16::from(*start.half());
        let end_half = i16::from(*self.half());
        let half_diff = end_half - start_half; // -1 if start is second half, +1 if end is first half, 0 if both are same half
        let elapsed = elapsed_ym + half_diff;
        assert!(elapsed >= 0, "Elapsed periods cannot be negative");
        elapsed as u16
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

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = YearMonthAndFortnight;

    #[test]
    fn elapsed_periods_since_same_is_zero() {
        let sut = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::First)
            .build();

        assert_eq!(sut.elapsed_periods_since(sut), 0);
    }

    #[test]
    fn elapsed_periods_since_same_month_different_half_is_1() {
        let early = Sut::builder()
            .year(2025.into())
            .month(Month::July)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::July)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(late.elapsed_periods_since(early), 1);
    }

    #[test]
    fn elapsed_periods_since_across_month_one_half_is_1() {
        let early = Sut::builder()
            .year(2025.into())
            .month(Month::June)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::July)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early), 1);
    }

    #[test]
    fn elapsed_periods_since_across_month_and_year_one_half_is_1() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::January)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early), 1);
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_is_second_half() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::March)
            .half(MonthHalf::First)
            .build();
        let expected = 1 + 2 + 2; // second half of 2024-12 plus whole 2025-01 and whole 2025-02
        assert_eq!(late.elapsed_periods_since(early), expected);
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_one_period() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::January)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early), 1); // Second half of December only
    }

    #[test]
    fn elapsed_periods_since_one_period_only() {
        let early = Sut::builder()
            .year(2025.into())
            .month(Month::May)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::June)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early), 1); // Second half of May only
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_is_first_half() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::March)
            .half(MonthHalf::First)
            .build();
        let expected = 2 + 2 + 2; // Whole of December, January and February
        assert_eq!(late.elapsed_periods_since(early), expected);
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_and_late_are_second_half() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::March)
            .half(MonthHalf::Second)
            .build();
        let expected = 1 + 2 + 2 + 1; // half Dec, half March, whole Jan/Feb
        assert_eq!(late.elapsed_periods_since(early), expected);
    }

    #[test]
    fn elapsed_periods_since_cross_year_early_is_first_half_and_late_is_second_half() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::March)
            .half(MonthHalf::Second)
            .build();
        let expected = 2 + 2 + 2 + 1; // whole Dec/Jan/Feb, half March
        assert_eq!(late.elapsed_periods_since(early), expected);
    }

    #[test]
    fn elapsed_periods_since_one_year_first_first_is_24() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early), 24);
    }

    #[test]
    fn elapsed_periods_since_one_year_second_second_is_24() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(late.elapsed_periods_since(early), 24);
    }

    #[test]
    fn elapsed_periods_since_one_year_second_first_is_23() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        assert_eq!(late.elapsed_periods_since(early), 23);
    }

    #[test]
    fn elapsed_periods_since_one_year_first_second_is_25() {
        let early = Sut::builder()
            .year(2024.into())
            .month(Month::December)
            .half(MonthHalf::First)
            .build();
        let late = Sut::builder()
            .year(2025.into())
            .month(Month::December)
            .half(MonthHalf::Second)
            .build();
        assert_eq!(late.elapsed_periods_since(early), 25);
    }
}
