use std::{borrow::Borrow, ops::Mul};

use crate::prelude::*;

impl Year {
    /// Checks if this year is a leap year, if it is, `true` is returned, else
    /// `false`
    pub fn is_leap(&self) -> bool {
        let year = **self;
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

impl Month {
    pub fn last_day(&self, is_leap_year: bool) -> Day {
        use Month::*;
        match self {
            January | March | May | July | August | October | December => {
                Day::try_from(31).expect("LEQ 31 days")
            }
            April | June | September | November => Day::try_from(30).expect("LEQ 31 days"),
            February => {
                if is_leap_year {
                    Day::try_from(29).expect("LEQ 31 days")
                } else {
                    Day::try_from(28).expect("LEQ 31 days")
                }
            }
        }
    }
}

impl YearMonthAndFortnight {
    pub fn current() -> Self {
        let today = chrono::Local::now().date_naive();
        Self::builder()
            .year(Year::from(today.year()))
            .month(Month::try_from(today.month() as i32).expect("Chrono should return valid month"))
            .half(MonthHalf::from(today))
            .build()
    }

    pub fn last() -> Self {
        Self::current().one_half_earlier()
    }

    pub fn one_half_earlier(&self) -> Self {
        match self.half() {
            MonthHalf::First => {
                let ym = YearAndMonth::from(*self);
                let ym = ym.one_month_earlier();
                Self::year_and_month_with_half(ym, MonthHalf::Second)
            }
            MonthHalf::Second => Self::builder()
                .year(*self.year())
                .month(*self.month())
                .half(MonthHalf::First)
                .build(),
        }
    }
}

impl YearAndMonth {
    /// Returns the last day of the month for this `YearAndMonth`, e.g. if the
    /// year is not a leap year, February will return 28, and for leap year
    /// 29 is returned.
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    ///
    /// let year_and_month = YearAndMonth::january(2025);
    /// assert_eq!(year_and_month.last_day_of_month(), Day::try_from(31).unwrap());
    /// ```
    pub fn last_day_of_month(&self) -> Day {
        self.month().last_day(self.year().is_leap())
    }

    /// Converts this `YearAndMonth` to a `Date` representing the last day of the month.
    ///
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let month = YearAndMonth::january(2025);
    /// let date = month.to_date_end_of_month();
    /// assert_eq!(date.year(), &Year::from(2025));
    /// assert_eq!(date.month(), &Month::January);
    /// assert_eq!(date.day(), &Day::try_from(31).unwrap());
    /// ```
    pub fn to_date_end_of_month(&self) -> Date {
        Date::builder()
            .year(*self.year())
            .month(*self.month())
            .day(self.last_day_of_month())
            .build()
    }

    pub fn current() -> Self {
        Self::from(YearMonthAndFortnight::current())
    }

    /// Returns a new `YearAndMonth` that is one month earlier than this one.
    /// If the month is January, it will return December of the previous year.
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let month = YearAndMonth::january(2025);
    /// let one_month_earlier = month.one_month_earlier();
    /// assert_eq!(one_month_earlier, YearAndMonth::december(2024));
    /// ```
    pub fn one_month_earlier(&self) -> Self {
        let mut year = **self.year();
        let mut month = **self.month();

        if month == 1 {
            year -= 1;
            month = 12
        } else {
            month -= 1
        }

        Self::builder()
            .year(Year::from(year))
            .month(Month::try_from(month).expect("Should return valid month"))
            .build()
    }

    /// Returns a new `YearAndMonth` that is one month later than this one - by
    /// reading the calendar - if the current month is December, it will return
    /// January of the next year.
    pub fn last() -> Self {
        Self::current().one_month_earlier()
    }

    /// Returns the number of months elapsed between this `YearAndMonth` and
    /// another `YearAndMonth`.
    ///
    /// # Examples
    /// ```
    /// extern crate klirr_core_invoice;
    /// use klirr_core_invoice::prelude::*;
    /// let start = YearAndMonth::january(2025);
    /// let end = YearAndMonth::april(2025);
    /// assert_eq!(end.elapsed_months_since(start).unwrap(), 3);
    /// ```
    ///
    /// # Throws
    /// Throws an error if the `start` month is after the `end` month.
    pub fn elapsed_months_since(&self, start: impl Borrow<Self>) -> Result<u16> {
        let end = self;
        let start = start.borrow();
        if start > end {
            return Err(Error::StartPeriodAfterEndPeriod {
                start: start.to_string(),
                end: end.to_string(),
            });
        }
        let start_year = **start.year();
        let start_month = **start.month() as u16;
        let end_year = **end.year();
        let end_month = **end.month() as u16;
        // When we perform arithmetic below we need to consider
        // e.g. Start: 2024-12, End: 2025-03, where start month can come later
        // in the year than the end month.
        let months_per_year = 12;
        let start_months = start_year * months_per_year + start_month;
        let end_months = end_year * months_per_year + end_month;
        let months_elapsed = end_months - start_months;
        Ok(months_elapsed)
    }
}

impl TryInto<YearAndMonth> for PeriodAnno {
    type Error = crate::Error;

    fn try_into(self) -> std::result::Result<YearAndMonth, Self::Error> {
        todo!()
    }
}

impl TryFromPeriodAnno for YearMonthAndFortnight {
    fn try_from_period_anno(period: PeriodAnno) -> Result<Self> {
        period
            .try_unwrap_year_month_and_fortnight()
            .map_err(|_| Error::PeriodIsNotYearMonthAndFortnight)
    }
}

impl TryFromPeriodAnno for YearAndMonth {
    fn try_from_period_anno(period: PeriodAnno) -> Result<Self> {
        period
            .try_unwrap_year_and_month()
            .map_err(|_| Error::PeriodIsNotYearAndMonth)
    }
}

impl IsPeriod for YearAndMonth {
    fn max_granularity(&self) -> Granularity {
        Granularity::Month
    }

    fn elapsed_periods_since(&self, start: impl Borrow<Self>) -> Result<u16> {
        self.elapsed_months_since(start)
    }

    fn to_date_end_of_period(&self) -> Date {
        self.to_date_end_of_month()
    }

    fn year(&self) -> &Year {
        self.year()
    }

    fn month(&self) -> &Month {
        self.month()
    }
}

/// Calculates the invoice number based on the offset, target month, whether
/// the items are expenses, and the months off record.
/// This function assumes that the `ProtoInvoiceInfo` has already been validated
/// to ensure that the target month is not in the record of months off.
/// It computes the invoice number by considering the elapsed months since
/// the offset month, adjusting for any months that are off record, and
/// adding an additional increment if the items are expenses.
///
/// ```
/// extern crate klirr_core_invoice;
/// use klirr_core_invoice::prelude::*;
/// let offset = TimestampedInvoiceNumber::<YearAndMonth>::builder().offset(100.into()).period(YearAndMonth::january(2024)).build();
/// let target_month = YearAndMonth::august(2024);
/// let is_expenses = true;
/// let months_off_record = RecordOfPeriodsOff::new([
///   YearAndMonth::march(2024),
///   YearAndMonth::april(2024),
/// ]);
/// let invoice_number = calculate_invoice_number(
///     &offset,
///     &target_month,
///     is_expenses,
///     &months_off_record,
/// ).unwrap();
///
/// // The expected invoice number is calculated as follows:
/// // - Offset is 100
/// // - Target month is August 2024, which is 7 months after January
/// // - Months off record are March and April, which are 2 months off
/// // - Since this is for expenses, we add 1 to the final invoice number.
/// // - Therefore, the invoice number should be 100 + 7 - 2 + 1 = 106
/// let expected = InvoiceNumber::from(106);
/// assert_eq!(invoice_number, expected);
/// ```
pub fn calculate_invoice_number<Period: IsPeriod>(
    offset: &TimestampedInvoiceNumber<Period>,
    target_period: &Period,
    is_expenses: bool,
    record_of_periods_off: &RecordOfPeriodsOff<Period>,
) -> Result<InvoiceNumber> {
    if record_of_periods_off.contains(offset.period()) {
        return Err(Error::RecordsOffMustNotContainOffsetPeriod {
            offset_period: format!("{:?}", offset.period()),
        });
    }
    let periods_elapsed_since_offset = target_period.elapsed_periods_since(offset.period())?;

    let mut periods_off_to_subtract = 0;
    for period_off in record_of_periods_off.iter() {
        if period_off > offset.period() && period_off <= target_period {
            // If the period is recorded as off, we need to adjust the invoice number
            // by subtracting the number of periods off
            periods_off_to_subtract += 1;
        }
    }
    let mut invoice_number =
        **offset.offset() + periods_elapsed_since_offset - periods_off_to_subtract;
    if is_expenses {
        // For expenses we add 1, ensuring that if we invoice for services and
        // expenses the same month, the expense invoice number is always higher.
        invoice_number += 1;
    }
    Ok(InvoiceNumber::from(invoice_number))
}

/// Calculates the number of working days in a given month, excluding weekends.
///
/// # Errors
/// Returns an error if the target month is in the record of months off.
///
/// ```
/// extern crate klirr_core_invoice;
/// use klirr_core_invoice::prelude::*;
///
/// let target_month = YearAndMonth::january(2024);
/// let working_days = quantity_in_period(&target_month, Granularity::Day, Cadence::Monthly, &RecordOfPeriodsOff::default());
/// assert_eq!(*working_days.unwrap(), dec!(23)); // January 2024 has 23
/// ```
pub fn quantity_in_period<Period: IsPeriod>(
    target_period: &Period,
    granularity: Granularity,
    cadence: Cadence,
    record_of_periods_off: &RecordOfPeriodsOff<Period>,
) -> Result<Quantity> {
    if record_of_periods_off.contains(target_period) {
        return Err(Error::TargetPeriodMustNotBeInRecordOfPeriodsOff {
            target_period: format!("{:?}", target_period),
        });
    }

    if granularity > target_period.max_granularity() {
        return Err(Error::GranularityTooCoarse {
            granularity,
            max_granularity: target_period.max_granularity(),
            target_period: format!("{:?}", target_period),
        });
    }

    let daily = || working_days_in_period(target_period);
    let hourly = || {
        // TODO maybe 8h per day should be configurable
        Result::Ok(Quantity::EIGHT.mul(*working_days_in_period(target_period)?))
    };

    match granularity {
        Granularity::Month => match cadence {
            Cadence::Monthly => Ok(Quantity::ONE),
            Cadence::BiWeekly => Err(Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly),
        },
        Granularity::Fortnight => match cadence {
            Cadence::Monthly => Ok(Quantity::TWO), // Two fortnights in a month
            Cadence::BiWeekly => Ok(Quantity::ONE),
        },
        Granularity::Day => daily(),
        Granularity::Hour => hourly(),
    }
}

trait FromYmd: Sized {
    fn ymd(year: impl Into<i32>, month: impl Into<u32>, day: impl Into<u32>) -> Result<Self>;
}
impl FromYmd for NaiveDate {
    fn ymd(year: impl Into<i32>, month: impl Into<u32>, day: impl Into<u32>) -> Result<Self> {
        let year = year.into();
        let month = month.into();
        let day = day.into();
        Self::from_ymd_opt(year, month, day)
            .ok_or(Error::InvalidDate {
                underlying: "Failed to create date from year and month".to_owned(),
            })?
            .pred_opt()
            .ok_or(Error::InvalidDate {
                underlying: "Failed to create date from year and month".to_owned(),
            })
    }
}

/// Calculates the number of working days in a given month, excluding weekends.
///
/// # Errors
/// Returns an error if the target month is in the record of months off.
fn working_days_in_period<Period: IsPeriod>(target_period: &Period) -> Result<Quantity> {
    let year = **target_period.year() as i32;
    let month = **target_period.month() as u32;

    // Start from the 1st of the month
    let mut day = NaiveDate::ymd(year, month, 1u32)?;

    // Get the last day of the month
    let last_day = if month == 12 {
        NaiveDate::ymd(year + 1, 1u32, 1u32)
    } else {
        NaiveDate::ymd(year, month + 1, 1u32)
    }?;

    let mut working_days = 0;
    while day <= last_day {
        match day.weekday() {
            Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => {
                working_days += 1;
            }
            _ => {}
        }
        day = day.succ_opt().ok_or(Error::InvalidDate {
            underlying: "Failed to get next day".to_owned(),
        })?
    }

    Ok(Quantity::from(working_days))
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    #![allow(unused)]
    use super::*;
    use test_log::test;

    /// 2025 is not a leap year
    const JAN_2025: YearAndMonth = YearAndMonth::january(2025);
    /// 2025 is not a leap year
    const FEB_2025: YearAndMonth = YearAndMonth::february(2025);
    /// 2025 is not a leap year
    const APR_2025: YearAndMonth = YearAndMonth::april(2025);
    /// 2025 is not a leap year
    const MAY_2025: YearAndMonth = YearAndMonth::may(2025);
    /// 2025 is not a leap year
    const JUNE_2025: YearAndMonth = YearAndMonth::june(2025);
    const JUNE_2025_FIRST: YearMonthAndFortnight =
        YearMonthAndFortnight::year_and_month_with_half(JUNE_2025, MonthHalf::First);
    /// 2025 is not a leap year
    const JULY_2025: YearAndMonth = YearAndMonth::july(2025);
    const JULY_2025_FIRST: YearMonthAndFortnight =
        YearMonthAndFortnight::year_and_month_with_half(JULY_2025, MonthHalf::First);
    /// 2025 is not a leap year
    const AUG_2025: YearAndMonth = YearAndMonth::august(2025);
    const AUG_2025_FIRST: YearMonthAndFortnight =
        YearMonthAndFortnight::year_and_month_with_half(AUG_2025, MonthHalf::First);
    /// 2025 is not a leap year
    const SEPT_2025: YearAndMonth = YearAndMonth::september(2025);
    const SEPT_2025_FIRST: YearMonthAndFortnight =
        YearMonthAndFortnight::year_and_month_with_half(SEPT_2025, MonthHalf::First);
    /// 2025 is not a leap year
    const OCT_2025: YearAndMonth = YearAndMonth::october(2025);
    /// 2025 is not a leap year
    const NOV_2025: YearAndMonth = YearAndMonth::november(2025);
    /// 2025 is not a leap year
    const DEC_2025: YearAndMonth = YearAndMonth::december(2025);
    const DEC_2025_FIRST: YearMonthAndFortnight =
        YearMonthAndFortnight::year_and_month_with_half(DEC_2025, MonthHalf::First);

    /// 2026 is not a leap year
    const JAN_2026: YearAndMonth = YearAndMonth::january(2026);
    /// 2026 is not a leap year
    const JUNE_2026: YearAndMonth = YearAndMonth::june(2026);
    /// 2026 is not a leap year
    const JULY_2026: YearAndMonth = YearAndMonth::july(2026);
    /// 2026 is not a leap year
    const AUG_2026: YearAndMonth = YearAndMonth::august(2026);

    /// 2028 is a leap year
    const JAN_2028: YearAndMonth = YearAndMonth::january(2028);
    /// 2028 is a leap year
    const FEB_2028: YearAndMonth = YearAndMonth::february(2028);
    /// 2028 is a leap year
    const MAR_2028: YearAndMonth = YearAndMonth::march(2028);

    #[test]
    fn test_quantity_in_period_year_and_month_with_month_granularity() {
        // YearAndMonth has max_granularity of Month, so Month granularity should work
        let target_period = JAN_2025;
        let record_of_periods_off = RecordOfPeriodsOff::new([]);

        let result = quantity_in_period(
            &target_period,
            Granularity::Month,
            Cadence::Monthly,
            &record_of_periods_off,
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Quantity::ONE);
    }

    #[test]
    fn test_quantity_in_period_year_and_month_with_day_granularity() {
        // Day granularity should work with YearAndMonth (less coarse than Month)
        let target_period = JAN_2025;
        let record_of_periods_off = RecordOfPeriodsOff::new([]);

        let result = quantity_in_period(
            &target_period,
            Granularity::Day,
            Cadence::Monthly,
            &record_of_periods_off,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_quantity_in_period_year_and_month_with_hour_granularity() {
        // Hour granularity should work with YearAndMonth (less coarse than Month)
        let target_period = JAN_2025;
        let record_of_periods_off = RecordOfPeriodsOff::new([]);

        let result = quantity_in_period(
            &target_period,
            Granularity::Hour,
            Cadence::Monthly,
            &record_of_periods_off,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_quantity_in_period_fortnight_with_month_granularity_should_fail() {
        // YearMonthAndFortnight has max_granularity of Fortnight
        // Month granularity should fail because it's coarser than Fortnight
        let fortnight_period = YearMonthAndFortnight::builder()
            .year(Year::from(2025))
            .month(Month::January)
            .half(MonthHalf::First)
            .build();
        let fortnight_record = RecordOfPeriodsOff::new([]);

        let result = quantity_in_period(
            &fortnight_period,
            Granularity::Month,
            Cadence::Monthly,
            &fortnight_record,
        );

        assert!(result.is_err());
        if let Err(Error::GranularityTooCoarse {
            granularity,
            max_granularity,
            ..
        }) = result
        {
            assert_eq!(granularity, Granularity::Month);
            assert_eq!(max_granularity, Granularity::Fortnight);
        } else {
            panic!("Expected GranularityTooCoarse error");
        }
    }

    #[test]
    fn test_quantity_in_period_fortnight_with_fortnight_granularity() {
        // Fortnight granularity should work for fortnight period (exactly the max granularity)
        let fortnight_period = YearMonthAndFortnight::builder()
            .year(Year::from(2025))
            .month(Month::January)
            .half(MonthHalf::First)
            .build();
        let fortnight_record = RecordOfPeriodsOff::new([]);

        let result = quantity_in_period(
            &fortnight_period,
            Granularity::Fortnight,
            Cadence::Monthly,
            &fortnight_record,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_quantity_in_period_fortnight_with_hour_granularity() {
        // Hour granularity should work for fortnight period (less coarse than Day)
        let fortnight_period = YearMonthAndFortnight::builder()
            .year(Year::from(2025))
            .month(Month::January)
            .half(MonthHalf::First)
            .build();
        let fortnight_record = RecordOfPeriodsOff::new([]);

        let result = quantity_in_period(
            &fortnight_period,
            Granularity::Hour,
            Cadence::Monthly,
            &fortnight_record,
        );

        assert!(result.is_ok());
    }

    fn test_invoice_number(
        offset_no: impl Into<InvoiceNumber>,
        offset_month: YearAndMonth,
        target_period: YearMonthAndFortnight,
        months_off: impl IntoIterator<Item = YearAndMonth>,
        is_expenses: bool,
        expected: impl Into<InvoiceNumber>,
    ) {
        let input = ValidInput::builder()
            .period(target_period)
            .items(if is_expenses {
                InvoicedItems::Expenses
            } else {
                InvoicedItems::Service { time_off: None }
            })
            .build();
        let information = ProtoInvoiceInfo::builder()
            .purchase_order(PurchaseOrder::from("PO"))
            .record_of_periods_off(RecordOfPeriodsOff::new(months_off))
            .offset(
                TimestampedInvoiceNumber::builder()
                    .offset(offset_no.into())
                    .period(offset_month)
                    .build(),
            )
            .build();

        let invoice_number = calculate_invoice_number(
            information.offset(),
            &YearAndMonth::from(target_period),
            is_expenses,
            information.record_of_periods_off(),
        )
        .unwrap();
        assert_eq!(invoice_number, expected.into());
    }

    mod services {
        use super::*;
        use test_log::test;

        mod no_months_off {

            use super::*;
            use test_log::test;

            #[test]
            fn when__target_month_eq_offset_month__then__invoice_num_eq_offset_num() {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025_FIRST,
                        [],
                        false,
                        invoice_no_offset,
                    );
                }
            }

            #[test]
            fn when__target_month_is_3_months_after_offset_month__then__invoice_num_eq_offset_num_plus_3()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        SEPT_2025_FIRST,
                        [],
                        false,
                        invoice_no_offset + 3,
                    );
                }
            }
        }

        mod months_off {

            use super::*;
            use test_log::test;

            #[test]
            fn when__target_month_eq_offset_month_and_months_off_is_in_past__then__invoice_num_eq_offset_num()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025_FIRST,
                        [APR_2025, MAY_2025],
                        false,
                        invoice_no_offset,
                    );
                }
            }

            #[test]
            fn when__target_month_eq_offset_month_and_months_off_is_in_future__then__invoice_num_eq_offset_num()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025_FIRST,
                        [JULY_2026, AUG_2026],
                        false,
                        invoice_no_offset,
                    );
                }
            }

            #[test]
            fn when__target_month_is_3_months_after_offset_month_with_all_months_off__then__invoice_num_eq_offset_num()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        JULY_2025_FIRST,
                        [MAY_2025, JUNE_2025, JULY_2025],
                        false,
                        invoice_no_offset,
                    );
                }
            }

            #[test]
            fn when__target_month_is_4_months_after_offset_month_with_3_months_off__then__invoice_num_eq_offset_num_plus_1()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        AUG_2025_FIRST,
                        [MAY_2025, JUNE_2025, JULY_2025],
                        false,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_is_8_months_after_offset_month_with_3_months_off__then__invoice_num_eq_offset_num_plus_5()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        DEC_2025_FIRST,
                        // Add some months off before offset_month and after target_month
                        [JAN_2025, MAY_2025, JULY_2025, SEPT_2025, JAN_2028],
                        false,
                        invoice_no_offset + 5,
                    );
                }
            }
        }
    }

    mod expenses {
        use super::*;
        use test_log::test;

        mod no_months_off {

            use super::*;
            use test_log::test;

            #[test]
            fn when__target_month_eq_offset_month__then__invoice_num_eq_offset_num_plus_1() {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025_FIRST,
                        [],
                        true,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_is_3_months_after_offset_month__then__invoice_num_eq_offset_num_plus_4()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        SEPT_2025_FIRST,
                        [],
                        true,
                        invoice_no_offset + 4,
                    );
                }
            }
        }

        mod months_off {

            use super::*;
            use test_log::test;

            #[test]
            fn when__target_month_eq_offset_month_and_months_off_is_in_past__then__invoice_num_eq_offset_num_plus_1()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025_FIRST,
                        [APR_2025, MAY_2025],
                        true,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_eq_offset_month_and_months_off_is_in_future__then__invoice_num_eq_offset_num_plus_1()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        JUNE_2025,
                        JUNE_2025_FIRST,
                        [JULY_2026, AUG_2026],
                        true,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_is_3_months_after_offset_month_with_all_months_off__then__invoice_num_eq_offset_num_plus_1()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        JULY_2025_FIRST,
                        [MAY_2025, JUNE_2025, JULY_2025],
                        true,
                        invoice_no_offset + 1,
                    );
                }
            }

            #[test]
            fn when__target_month_is_4_months_after_offset_month_with_3_months_off__then__invoice_num_eq_offset_num_plus_2()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        AUG_2025_FIRST,
                        [MAY_2025, JUNE_2025, JULY_2025],
                        true,
                        invoice_no_offset + 2,
                    );
                }
            }

            #[test]
            fn when__target_month_is_8_months_after_offset_month_with_3_months_off__then__invoice_num_eq_offset_num_plus_6()
             {
                for invoice_no_offset in 0..10 {
                    test_invoice_number(
                        invoice_no_offset,
                        APR_2025,
                        DEC_2025_FIRST,
                        // Add some months off before offset_month and after target_month
                        [JAN_2025, MAY_2025, JULY_2025, SEPT_2025, JAN_2028],
                        true,
                        invoice_no_offset + 6,
                    );
                }
            }
        }
    }

    #[test]
    fn test_last_day_of_month() {
        assert_eq!(JAN_2025.last_day_of_month(), Day::try_from(31).unwrap());
        assert_eq!(FEB_2028.last_day_of_month(), Day::try_from(29).unwrap());
        assert_eq!(MAR_2028.last_day_of_month(), Day::try_from(31).unwrap());
        assert_eq!(APR_2025.last_day_of_month(), Day::try_from(30).unwrap());
        assert_eq!(MAY_2025.last_day_of_month(), Day::try_from(31).unwrap());
        assert_eq!(JUNE_2025.last_day_of_month(), Day::try_from(30).unwrap());
        assert_eq!(JULY_2025.last_day_of_month(), Day::try_from(31).unwrap());
        assert_eq!(AUG_2025.last_day_of_month(), Day::try_from(31).unwrap());
        assert_eq!(SEPT_2025.last_day_of_month(), Day::try_from(30).unwrap());
        assert_eq!(OCT_2025.last_day_of_month(), Day::try_from(31).unwrap());
        assert_eq!(NOV_2025.last_day_of_month(), Day::try_from(30).unwrap());
        assert_eq!(DEC_2025.last_day_of_month(), Day::try_from(31).unwrap());

        assert_eq!(FEB_2025.last_day_of_month(), Day::try_from(28).unwrap());
    }

    #[test]
    fn test_elapsed_months_since_when_start_month_is_later_in_the_year_than_end_month() {
        let start = YearAndMonth::december(2024);
        let end = YearAndMonth::april(2025);
        assert_eq!(end.elapsed_months_since(start).unwrap(), 4);
        assert!(start < end);
    }

    #[test]
    fn test_one_month_earlier_of_january() {
        let january_2025 = YearAndMonth::january(2025);
        let december_2024 = YearAndMonth::december(2024);
        assert_eq!(january_2025.one_month_earlier(), december_2024);
    }

    #[test]
    fn elapsed_months_since_throws_when_start_month_is_later_than_end_month() {
        let start = YearAndMonth::april(2025);
        let end = YearAndMonth::march(2025);
        assert!(end.elapsed_months_since(start).is_err());
    }

    #[test]
    fn test_calculate_invoice_number_throws_for_invalid_input() {
        let month = YearAndMonth::may(2025);
        let invoice_info = ProtoInvoiceInfo::builder()
            .offset(
                TimestampedInvoiceNumber::builder()
                    .period(month)
                    .offset(237.into())
                    .build(),
            )
            .record_of_periods_off(RecordOfPeriodsOff::new([month]))
            .purchase_order(PurchaseOrder::sample())
            .build();

        let result = calculate_invoice_number(
            invoice_info.offset(),
            &YearAndMonth::december(2025),
            true,
            invoice_info.record_of_periods_off(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn quantity_in_period_target_month_is_in_record_of_months_off() {
        let target_month = YearAndMonth::january(2024);
        let months_off_record = RecordOfPeriodsOff::new([target_month]);
        let result = quantity_in_period(
            &target_month,
            Granularity::Day,
            Cadence::Monthly,
            &months_off_record,
        );
        assert!(result.is_err());
    }

    #[test]
    fn quantity_in_period_target_month_december() {
        let target_month = YearAndMonth::december(2025);
        let months_off_record = RecordOfPeriodsOff::new([]);
        let result = quantity_in_period(
            &target_month,
            Granularity::Day,
            Cadence::Monthly,
            &months_off_record,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn try_from_period_anno_for_year_and_month() {
        let sut = YearAndMonth::sample();
        let period_anno = PeriodAnno::YearAndMonth(sut);
        let outcome = YearAndMonth::try_from_period_anno(period_anno).unwrap();
        assert_eq!(outcome, sut);
    }

    #[test]
    fn try_from_period_anno_for_year_month_and_fortnight() {
        let sut = YearMonthAndFortnight::sample();
        let period_anno = PeriodAnno::YearMonthAndFortnight(sut);
        let outcome = YearMonthAndFortnight::try_from_period_anno(period_anno).unwrap();
        assert_eq!(outcome, sut);
    }

    #[test]
    fn test_quantity_in_period_throws_when_cadence_is_biweekly_and_granularity_is_month() {
        let target_period = YearAndMonth::january(2025);
        let record_of_periods_off = RecordOfPeriodsOff::new([]);
        let result = quantity_in_period(
            &target_period,
            Granularity::Month,
            Cadence::BiWeekly,
            &record_of_periods_off,
        );
        assert!(result.is_err());
        if let Err(Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly) = result {
            // Expected error
        } else {
            panic!("Expected CannotInvoiceForMonthWhenCadenceIsBiWeekly error");
        }
    }

    #[test]
    fn test_quantity_in_period_granularity_fortnight_cadence_monthly_is_two() {
        let target_period = YearAndMonth::january(2025);
        let record_of_periods_off = RecordOfPeriodsOff::new([]);
        let result = quantity_in_period(
            &target_period,
            Granularity::Fortnight,
            Cadence::Monthly,
            &record_of_periods_off,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Quantity::TWO);
    }

    #[test]
    fn test_quantity_in_period_granularity_fortnight_cadence_biweekly_is_one() {
        let target_period = YearAndMonth::january(2025);
        let record_of_periods_off = RecordOfPeriodsOff::new([]);
        let result = quantity_in_period(
            &target_period,
            Granularity::Fortnight,
            Cadence::BiWeekly,
            &record_of_periods_off,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Quantity::ONE);
    }

    #[test]
    fn ymf_one_half_earlier_month_half_first_half() {
        let ym = YearAndMonth::january(2025);
        let ymf = YearMonthAndFortnight::year_and_month_with_half(ym, MonthHalf::First);
        let one_half_earlier = ymf.one_half_earlier();
        assert_eq!(
            one_half_earlier,
            YearMonthAndFortnight::year_and_month_with_half(
                YearAndMonth::december(2024),
                MonthHalf::Second
            )
        );
    }

    #[test]
    fn ymf_one_half_earlier_month_half_second_half() {
        let ym = YearAndMonth::january(2025);
        let ymf = YearMonthAndFortnight::year_and_month_with_half(ym, MonthHalf::Second);
        let one_half_earlier = ymf.one_half_earlier();
        assert_eq!(
            one_half_earlier,
            YearMonthAndFortnight::year_and_month_with_half(ym, MonthHalf::First)
        );
    }
}
