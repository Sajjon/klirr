use std::borrow::Borrow;

use crate::prelude::*;

impl YearAndMonth {
    /// Returns the last day of the month for this `YearAndMonth`, e.g. if the
    /// year is not a leap year, February will return 28, and for leap year
    /// 29 is returned.
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    ///
    /// let year_and_month = YearAndMonth::january(2025);
    /// assert_eq!(year_and_month.last_day_of_month(), Day::try_from(31).unwrap());
    /// ```
    pub fn last_day_of_month(&self) -> Day {
        match **self.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Day::try_from(31).expect("LEQ 31 days"),
            4 | 6 | 9 | 11 => Day::try_from(30).expect("LEQ 31 days"),
            2 => {
                let year = **self.year();
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    Day::try_from(29).expect("LEQ 31 days")
                } else {
                    Day::try_from(28).expect("LEQ 31 days")
                }
            }
            _ => unreachable!("Invalid month value"),
        }
    }

    /// Converts this `YearAndMonth` to a `Date` representing the last day of the month.
    ///
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
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
        let today = chrono::Local::now().date_naive();
        Self::builder()
            .year(Year::from(today.year()))
            .month(Month::try_from(today.month() as i32).expect("Chrono should return valid month"))
            .build()
    }

    /// Returns a new `YearAndMonth` that is one month earlier than this one.
    /// If the month is January, it will return December of the previous year.
    /// ```
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
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
    /// extern crate klirr_core;
    /// use klirr_core::prelude::*;
    /// let start = YearAndMonth::january(2025);
    /// let end = YearAndMonth::april(2025);
    /// assert_eq!(end.elapsed_months_since(start), 3);
    /// ```
    ///
    /// # Panics
    /// Panics if the `start` month is after the `end` month.
    pub fn elapsed_months_since(&self, start: impl Borrow<Self>) -> u16 {
        let end = self;
        let start = start.borrow();
        assert!(start <= end, "Expected start <= end month");
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
        end_months - start_months
    }
}

impl ValidInput {
    /// Calculates the invoice number for the given `ProtoInvoiceInfo` based on
    /// the target month and whether the items are expenses or services.
    ///
    /// See `calculate_invoice_number` for the logic.
    pub fn invoice_number(&self, information: &ProtoInvoiceInfo) -> InvoiceNumber {
        let items = self.items();
        let target_month = self.month();
        let is_expenses = items.is_expenses();
        calculate_invoice_number(
            information.offset(),
            target_month,
            is_expenses,
            information.months_off_record(),
        )
    }
}

/// Calculates the invoice number based on the offset, target month, whether
/// the items are expenses, and the months off record.
/// This function assumes that the `ProtoInvoiceInfo` has already been validated
/// to ensure that the target month is not in the record of months off.
/// /// It computes the invoice number by considering the elapsed months since
/// the offset month, adjusting for any months that are off record, and
/// adding an additional increment if the items are expenses.
///
/// ```
/// extern crate klirr_core;
/// use klirr_core::prelude::*;
/// let offset = TimestampedInvoiceNumber::builder().offset(100.into()).month(YearAndMonth::january(2024)).build();
/// let target_month = YearAndMonth::august(2024);
/// let is_expenses = true;
/// let months_off_record = MonthsOffRecord::new([
///   YearAndMonth::march(2024),
///   YearAndMonth::april(2024),
/// ]);
/// let invoice_number = calculate_invoice_number(
///     &offset,
///     &target_month,
///     is_expenses,
///     &months_off_record,
/// );
///
/// /// The expected invoice number is calculated as follows:
/// /// - Offset is 100
/// /// - Target month is August 2024, which is 7 months after January
/// /// - Months off record are March and April, which are 2 months off
/// /// - Since this is for expenses, we add 1 to the final invoice number.
/// /// - Therefore, the invoice number should be 100 + 7 - 2 + 1 = 106
/// let expected = InvoiceNumber::from(106);
/// assert_eq!(invoice_number, expected);
/// ```
pub fn calculate_invoice_number(
    offset: &TimestampedInvoiceNumber,
    target_month: &YearAndMonth,
    is_expenses: bool,
    months_off_record: &MonthsOffRecord,
) -> InvoiceNumber {
    assert!(
        !months_off_record.contains(offset.month()),
        "Should have validated ProtoInvoiceInfo before calling this function"
    );
    let months_elapsed_since_offset = target_month.elapsed_months_since(offset.month());

    let mut months_off_to_subtract = 0;
    for year_and_month in months_off_record.iter() {
        if year_and_month > offset.month() && year_and_month <= target_month {
            // If the month is off record, we need to adjust the invoice number
            // by subtracting the number of months off record.
            months_off_to_subtract += 1;
        }
    }
    let mut invoice_number =
        **offset.offset() + months_elapsed_since_offset - months_off_to_subtract;
    if is_expenses {
        // For expenses we add 1, ensuring that if we invoice for services and
        // expenses the same month, the expense invoice number is always higher.
        invoice_number += 1;
    }
    InvoiceNumber::from(invoice_number)
}

/// Calculates the number of working days in a given month, excluding weekends.
///
/// # Errors
/// Returns an error if the target month is in the record of months off.
///
/// ```
/// extern crate klirr_core;
/// use klirr_core::prelude::*;
///
/// let target_month = YearAndMonth::january(2024);
/// let months_off_record = MonthsOffRecord::new([]);
/// let working_days = working_days_in_month(&target_month, &months_off_record);
/// assert_eq!(working_days.unwrap(), 23); // January 2024 has 23
/// ```
pub fn working_days_in_month(
    target_month: &YearAndMonth,
    months_off_record: &MonthsOffRecord,
) -> Result<u8> {
    if months_off_record.contains(target_month) {
        return Err(Error::TargetMonthMustNotBeInRecordOfMonthsOff {
            target_month: *target_month,
        });
    }

    let year = **target_month.year() as i32;
    let month = **target_month.month() as u32;

    // Start from the 1st of the month
    let mut day = NaiveDate::from_ymd_opt(year, month, 1)
        .expect("Should always be able to create a date from year, month (and day)");

    // Get the last day of the month
    let last_day = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    };

    let mut working_days = 0;
    while day <= last_day {
        match day.weekday() {
            Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => {
                working_days += 1;
            }
            _ => {}
        }
        day = day.succ_opt().unwrap();
    }

    Ok(working_days)
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
    /// 2025 is not a leap year
    const JULY_2025: YearAndMonth = YearAndMonth::july(2025);
    /// 2025 is not a leap year
    const AUG_2025: YearAndMonth = YearAndMonth::august(2025);
    /// 2025 is not a leap year
    const SEPT_2025: YearAndMonth = YearAndMonth::september(2025);
    /// 2025 is not a leap year
    const OCT_2025: YearAndMonth = YearAndMonth::october(2025);
    /// 2025 is not a leap year
    const NOV_2025: YearAndMonth = YearAndMonth::november(2025);
    /// 2025 is not a leap year
    const DEC_2025: YearAndMonth = YearAndMonth::december(2025);

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

    fn test_invoice_number(
        offset_no: impl Into<InvoiceNumber>,
        offset_month: YearAndMonth,
        target_month: YearAndMonth,
        months_off: impl IntoIterator<Item = YearAndMonth>,
        is_expenses: bool,
        expected: impl Into<InvoiceNumber>,
    ) {
        let input = ValidInput::builder()
            .month(target_month)
            .items(if is_expenses {
                InvoicedItems::Expenses
            } else {
                InvoicedItems::Service { days_off: None }
            })
            .build();
        let information = ProtoInvoiceInfo::builder()
            .purchase_order(PurchaseOrder::from("PO"))
            .months_off_record(MonthsOffRecord::new(months_off))
            .offset(
                TimestampedInvoiceNumber::builder()
                    .offset(offset_no.into())
                    .month(offset_month)
                    .build(),
            )
            .build();

        let invoice_number = input.invoice_number(&information);
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
                        JUNE_2025,
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
                        SEPT_2025,
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
                        JUNE_2025,
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
                        JUNE_2025,
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
                        JULY_2025,
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
                        AUG_2025,
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
                        DEC_2025,
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
                        JUNE_2025,
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
                        SEPT_2025,
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
                        JUNE_2025,
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
                        JUNE_2025,
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
                        JULY_2025,
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
                        AUG_2025,
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
                        DEC_2025,
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
        assert_eq!(end.elapsed_months_since(start), 4);
        assert!(start < end);
    }

    #[test]
    fn test_one_month_earlier_of_january() {
        let january_2025 = YearAndMonth::january(2025);
        let december_2024 = YearAndMonth::december(2024);
        assert_eq!(january_2025.one_month_earlier(), december_2024);
    }

    #[test]
    #[should_panic]
    fn test_elapsed_months_since_panic() {
        let start = YearAndMonth::april(2025);
        let end = YearAndMonth::march(2025);
        end.elapsed_months_since(start);
    }

    #[test]
    #[should_panic(
        expected = "Should have validated ProtoInvoiceInfo before calling this function"
    )]
    fn test_calculate_invoice_number_panics_for_invalid_input() {
        let month = YearAndMonth::may(2025);
        let invoice_info = ProtoInvoiceInfo::builder()
            .offset(
                TimestampedInvoiceNumber::builder()
                    .month(month)
                    .offset(237.into())
                    .build(),
            )
            .months_off_record(MonthsOffRecord::new([month]))
            .purchase_order(PurchaseOrder::sample())
            .build();

        let _ = calculate_invoice_number(
            invoice_info.offset(),
            &YearAndMonth::december(2025),
            true,
            invoice_info.months_off_record(),
        );
    }

    #[test]
    fn test_working_days_in_month_target_month_is_in_record_of_months_off() {
        let target_month = YearAndMonth::january(2024);
        let months_off_record = MonthsOffRecord::new([target_month]);
        let result = working_days_in_month(&target_month, &months_off_record);
        assert!(result.is_err());
    }

    #[test]
    fn test_working_days_in_month_target_month_december() {
        let target_month = YearAndMonth::december(2025);
        let months_off_record = MonthsOffRecord::new([]);
        let result = working_days_in_month(&target_month, &months_off_record);
        assert!(result.is_ok());
    }
}
