use std::str::FromStr;
use std::{cmp::Ordering, ops::Mul};

use crate::{
    Cadence, Date, Day, Error, Granularity, InvoiceNumber, Month, Quantity, RecordOfPeriodsOff,
    RelativeTime, Result, TimestampedInvoiceNumber, Year,
};
use chrono::{Datelike, NaiveDate, Weekday};

fn month_end_date(year: Year, month: Month) -> Date {
    Date::builder()
        .year(year)
        .month(month)
        .day(month.last_day(year))
        .build()
}

fn first_half_end_date(year: Year, month: Month) -> Date {
    let day = if month == Month::February { 14 } else { 15 };
    Date::builder()
        .year(year)
        .month(month)
        .day(Day::try_from(day).expect("valid day"))
        .build()
}

fn period_end_for_unit(date: Date, unit: Granularity) -> Result<Date> {
    match unit {
        Granularity::Month => Ok(date.end_of_month()),
        Granularity::Fortnight => {
            let first_half_end = first_half_end_date(*date.year(), *date.month());
            if date.day() <= first_half_end.day() {
                Ok(first_half_end)
            } else {
                Ok(date.end_of_month())
            }
        }
        unsupported => Err(Error::InvalidPeriod {
            bad_value: format!(
                "Unsupported relative period unit '{unsupported:?}', expected Month or Fortnight"
            ),
        }),
    }
}

fn period_end_for_cadence(date: Date, cadence: Cadence) -> Result<Date> {
    period_end_for_unit(date, cadence.max_granularity())
}

/// Normalizes a date to the cadence-aligned period-end date.
///
/// # Examples
/// ```
/// extern crate klirr_core_invoice;
/// use klirr_core_invoice::*;
///
/// let normalized = normalize_period_end_date_for_cadence(
///     "2025-05-12".parse::<Date>().unwrap(),
///     Cadence::Monthly,
/// )
/// .unwrap();
///
/// assert_eq!(normalized.to_string(), "2025-05-31");
/// ```
pub fn normalize_period_end_date_for_cadence(date: Date, cadence: Cadence) -> Result<Date> {
    period_end_for_cadence(date, cadence)
}

fn month_serial(period_end: Date) -> i32 {
    let year = **period_end.year() as i32;
    let month_zero_based = *period_end.month() as i32 - 1;
    year * 12 + month_zero_based
}

fn fortnight_serial(period_end: Date) -> Result<i32> {
    let period_end = period_end_for_unit(period_end, Granularity::Fortnight)?;
    let first_half_end = first_half_end_date(*period_end.year(), *period_end.month());
    let half = if period_end.day() == first_half_end.day() {
        0
    } else {
        1
    };
    let year = **period_end.year() as i32;
    let month_zero_based = *period_end.month() as i32 - 1;
    Ok(year * 24 + month_zero_based * 2 + half)
}

fn shift_period_end(period_end: Date, unit: Granularity, amount: i16) -> Result<Date> {
    let offset = amount as i32;
    match unit {
        Granularity::Month => {
            let start = period_end_for_unit(period_end, Granularity::Month)?;
            let serial = month_serial(start) + offset;
            if serial < 0 {
                return Err(Error::InvalidDate {
                    underlying: "negative period serial".to_owned(),
                });
            }
            let year = Year::from(serial / 12);
            let month = Month::try_from((serial % 12 + 1) as i32)?;
            Ok(month_end_date(year, month))
        }
        Granularity::Fortnight => {
            let start = period_end_for_unit(period_end, Granularity::Fortnight)?;
            let serial = fortnight_serial(start)? + offset;
            if serial < 0 {
                return Err(Error::InvalidDate {
                    underlying: "negative period serial".to_owned(),
                });
            }
            let year = Year::from(serial / 24);
            let remainder = serial % 24;
            let month = Month::try_from((remainder / 2 + 1) as i32)?;
            let half = remainder % 2;
            if half == 0 {
                Ok(first_half_end_date(year, month))
            } else {
                Ok(month_end_date(year, month))
            }
        }
        unsupported => Err(Error::InvalidPeriod {
            bad_value: format!(
                "Unsupported relative period unit '{unsupported:?}', expected Month or Fortnight"
            ),
        }),
    }
}

fn elapsed_periods_since(start: Date, end: Date, cadence: Cadence) -> Result<u16> {
    let start = period_end_for_cadence(start, cadence)?;
    let end = period_end_for_cadence(end, cadence)?;
    if start > end {
        return Err(Error::StartPeriodAfterEndPeriod {
            start: start.to_string(),
            end: end.to_string(),
        });
    }

    let elapsed = match cadence {
        Cadence::Monthly => month_serial(end) - month_serial(start),
        Cadence::BiWeekly => fortnight_serial(end)? - fortnight_serial(start)?,
    };

    Ok(elapsed as u16)
}

/// Converts a relative time (e.g. current/last month or fortnight) into a period-end date.
///
/// # Examples
/// ```
/// extern crate klirr_core_invoice;
/// use klirr_core_invoice::*;
///
/// let current_month = period_end_from_relative_time(RelativeTime::current(Granularity::Month)).unwrap();
/// let last_month = period_end_from_relative_time(RelativeTime::last(Granularity::Month)).unwrap();
///
/// assert!(last_month < current_month);
/// ```
pub fn period_end_from_relative_time(relative: RelativeTime) -> Result<Date> {
    let current_period_end = period_end_for_unit(Date::today(), *relative.unit())?;
    shift_period_end(current_period_end, *relative.unit(), *relative.amount())
}

fn parse_legacy_period_label(value: &str) -> Result<(Date, Granularity)> {
    let mut parts = value.splitn(3, '-');
    let year_part = parts.next().expect("splitn always returns a first segment");
    let Some(month_part) = parts.next() else {
        return Err(Error::InvalidPeriod {
            bad_value: value.to_owned(),
        });
    };

    let year = Year::from_str(year_part)?;
    let month = Month::from_str(month_part)?;

    if let Some(rest) = parts.next() {
        let date = match rest {
            "first" | "first-half" | "1" => first_half_end_date(year, month),
            "second" | "second-half" | "2" => month_end_date(year, month),
            _ => {
                return Err(Error::InvalidPeriod {
                    bad_value: value.to_owned(),
                });
            }
        };
        return Ok((date, Granularity::Fortnight));
    }

    Ok((month_end_date(year, month), Granularity::Month))
}

/// Parses a user-facing period label into a period-end date using cadence rules.
///
/// Supports legacy labels (`YYYY-MM`, `YYYY-MM-first-half`) and full dates (`YYYY-MM-DD`).
///
/// # Examples
/// ```
/// extern crate klirr_core_invoice;
/// use klirr_core_invoice::*;
///
/// let monthly = parse_period_label_for_cadence("2025-05", Cadence::Monthly).unwrap();
/// assert_eq!(monthly.to_string(), "2025-05-31");
///
/// let fortnight = parse_period_label_for_cadence("2025-02-first-half", Cadence::BiWeekly).unwrap();
/// assert_eq!(fortnight.to_string(), "2025-02-14");
/// ```
pub fn parse_period_label_for_cadence(value: &str, cadence: Cadence) -> Result<Date> {
    if let Ok((date, unit)) = parse_legacy_period_label(value) {
        return match (cadence, unit) {
            (Cadence::Monthly, Granularity::Fortnight) => {
                Err(Error::CannotExpenseForFortnightWhenCadenceIsMonthly)
            }
            (Cadence::BiWeekly, Granularity::Month) => {
                Err(Error::CannotExpenseForMonthWhenCadenceIsBiWeekly)
            }
            _ => period_end_for_cadence(date, cadence),
        };
    }

    let date = Date::from_str(value)?;
    period_end_for_cadence(date, cadence)
}

/// Calculates the invoice number from an offset and target period-end date.
///
/// # Examples
/// ```
/// extern crate klirr_core_invoice;
/// use klirr_core_invoice::*;
///
/// let offset = TimestampedInvoiceNumber::builder()
///     .offset(100)
///     .date("2024-01-31".parse::<Date>().unwrap())
///     .build();
/// let target_period_end_date = "2024-08-31".parse::<Date>().unwrap();
/// let periods_off = RecordOfPeriodsOff::new([
///     "2024-03-31".parse::<Date>().unwrap(),
///     "2024-04-30".parse::<Date>().unwrap(),
/// ]);
///
/// let invoice_number = calculate_invoice_number(
///     &offset,
///     &target_period_end_date,
///     Cadence::Monthly,
///     true,
///     &periods_off,
/// )
/// .unwrap();
///
/// assert_eq!(invoice_number, InvoiceNumber::from(106));
/// ```
pub fn calculate_invoice_number(
    offset: &TimestampedInvoiceNumber,
    target_date: &Date,
    cadence: Cadence,
    is_expenses: bool,
    record_of_periods_off: &RecordOfPeriodsOff,
) -> Result<InvoiceNumber> {
    let offset_date = period_end_for_cadence(*offset.date(), cadence)?;
    if record_of_periods_off.contains(&offset_date) {
        return Err(Error::RecordsOffMustNotContainOffsetPeriod {
            offset_period: offset_date.to_string(),
        });
    }

    let target_date = period_end_for_cadence(*target_date, cadence)?;
    let periods_elapsed_since_offset = elapsed_periods_since(offset_date, target_date, cadence)?;

    let mut periods_off_to_subtract = 0u16;
    for period_off in record_of_periods_off.iter() {
        let period_off = period_end_for_cadence(*period_off, cadence)?;
        if period_off > offset_date && period_off <= target_date {
            periods_off_to_subtract += 1;
        }
    }

    let mut invoice_number =
        **offset.offset() + periods_elapsed_since_offset - periods_off_to_subtract;

    if is_expenses {
        invoice_number += 1;
    }

    Ok(InvoiceNumber::from(invoice_number))
}

fn period_bounds(period_end: Date, cadence: Cadence) -> Result<(Date, Date)> {
    let period_end = period_end_for_cadence(period_end, cadence)?;
    let start = match cadence {
        Cadence::Monthly => Date::builder()
            .year(*period_end.year())
            .month(*period_end.month())
            .day(Day::try_from(1).expect("1 is a valid day"))
            .build(),
        Cadence::BiWeekly => {
            let first_half_end = first_half_end_date(*period_end.year(), *period_end.month());
            match period_end.day().cmp(first_half_end.day()) {
                Ordering::Less | Ordering::Equal => Date::builder()
                    .year(*period_end.year())
                    .month(*period_end.month())
                    .day(Day::try_from(1).expect("1 is a valid day"))
                    .build(),
                Ordering::Greater => Date::builder()
                    .year(*period_end.year())
                    .month(*period_end.month())
                    .day(Day::try_from(i32::from(**first_half_end.day()) + 1).expect("valid day"))
                    .build(),
            }
        }
    };

    Ok((start, period_end))
}

fn working_days_between(start: Date, end: Date) -> Result<Quantity> {
    let start = NaiveDate::from_ymd_opt(
        **start.year() as i32,
        **start.month() as u32,
        **start.day() as u32,
    )
    .ok_or(Error::InvalidDate {
        underlying: "Invalid start date".to_owned(),
    })?;

    let end = NaiveDate::from_ymd_opt(
        **end.year() as i32,
        **end.month() as u32,
        **end.day() as u32,
    )
    .ok_or(Error::InvalidDate {
        underlying: "Invalid end date".to_owned(),
    })?;

    let mut current = start;
    let mut working_days = 0;
    while current <= end {
        match current.weekday() {
            Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => {
                working_days += 1;
            }
            Weekday::Sat | Weekday::Sun => {}
        }
        current = current.succ_opt().ok_or(Error::InvalidDate {
            underlying: "Failed to advance day".to_owned(),
        })?;
    }

    Ok(Quantity::from(working_days))
}

fn working_days_in_period(period_end: Date, cadence: Cadence) -> Result<Quantity> {
    let (start, end) = period_bounds(period_end, cadence)?;
    working_days_between(start, end)
}

/// Calculates billable quantity for a period-end date and cadence.
///
/// # Examples
/// ```
/// extern crate klirr_core_invoice;
/// use klirr_core_invoice::*;
/// use rust_decimal::dec;
///
/// let target_period_end_date = "2024-01-31".parse::<Date>().unwrap();
/// let quantity = quantity_in_period(
///     &target_period_end_date,
///     Granularity::Day,
///     Cadence::Monthly,
///     &RecordOfPeriodsOff::default(),
/// )
/// .unwrap();
///
/// assert_eq!(*quantity, dec!(23));
/// ```
pub fn quantity_in_period(
    target_date: &Date,
    granularity: Granularity,
    cadence: Cadence,
    record_of_periods_off: &RecordOfPeriodsOff,
) -> Result<Quantity> {
    let target_date = period_end_for_cadence(*target_date, cadence)?;

    if record_of_periods_off.contains(&target_date) {
        return Err(Error::TargetPeriodMustNotBeInRecordOfPeriodsOff {
            target_period: target_date.to_string(),
        });
    }

    if matches!(
        (granularity, cadence),
        (Granularity::Month, Cadence::BiWeekly)
    ) {
        return Err(Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly);
    }

    match granularity {
        Granularity::Month => Ok(Quantity::ONE),
        Granularity::Fortnight => match cadence {
            Cadence::Monthly => Ok(Quantity::TWO),
            Cadence::BiWeekly => Ok(Quantity::ONE),
        },
        Granularity::Day => working_days_in_period(target_date, cadence),
        Granularity::Hour => {
            Ok(Quantity::EIGHT.mul(*working_days_in_period(target_date, cadence)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;
    use rust_decimal::dec;
    use std::str::FromStr;

    fn d(s: &str) -> Date {
        Date::from_str(s).unwrap()
    }

    #[test]
    fn period_end_for_unit_month_and_fortnight() {
        assert_eq!(
            period_end_for_unit(d("2025-05-12"), Granularity::Month).unwrap(),
            d("2025-05-31")
        );
        assert_eq!(
            period_end_for_unit(d("2025-05-12"), Granularity::Fortnight).unwrap(),
            d("2025-05-15")
        );
        assert_eq!(
            period_end_for_unit(d("2025-05-20"), Granularity::Fortnight).unwrap(),
            d("2025-05-31")
        );
    }

    #[test]
    fn period_end_for_unit_rejects_unsupported_granularity() {
        let result = period_end_for_unit(d("2025-05-31"), Granularity::Day);
        assert!(matches!(result, Err(Error::InvalidPeriod { .. })));
    }

    #[test]
    fn normalize_period_end_date_for_cadence_biweekly() {
        let normalized =
            normalize_period_end_date_for_cadence(d("2025-05-16"), Cadence::BiWeekly).unwrap();
        assert_eq!(normalized, d("2025-05-31"));
    }

    #[test]
    fn shift_period_end_month_and_fortnight() {
        assert_eq!(
            shift_period_end(d("2025-01-31"), Granularity::Month, 1).unwrap(),
            d("2025-02-28")
        );
        assert_eq!(
            shift_period_end(d("2025-01-15"), Granularity::Fortnight, 1).unwrap(),
            d("2025-01-31")
        );
        assert_eq!(
            shift_period_end(d("2025-01-31"), Granularity::Fortnight, 1).unwrap(),
            d("2025-02-14")
        );
    }

    #[test]
    fn shift_period_end_rejects_invalid_or_unsupported() {
        let negative_month = shift_period_end(d("0001-01-31"), Granularity::Month, -20);
        assert_eq!(
            negative_month.unwrap_err(),
            Error::InvalidDate {
                underlying: "negative period serial".to_owned()
            }
        );

        let negative_fortnight = shift_period_end(d("0001-01-14"), Granularity::Fortnight, -40);
        assert_eq!(
            negative_fortnight.unwrap_err(),
            Error::InvalidDate {
                underlying: "negative period serial".to_owned()
            }
        );

        let unsupported = shift_period_end(d("2025-01-31"), Granularity::Day, 0);
        assert!(matches!(unsupported, Err(Error::InvalidPeriod { .. })));
    }

    #[test]
    fn elapsed_periods_since_monthly_and_biweekly() {
        assert_eq!(
            elapsed_periods_since(d("2025-01-31"), d("2025-03-31"), Cadence::Monthly).unwrap(),
            2
        );
        assert_eq!(
            elapsed_periods_since(d("2025-05-15"), d("2025-06-30"), Cadence::BiWeekly).unwrap(),
            3
        );
    }

    #[test]
    fn elapsed_periods_since_fails_when_start_after_end() {
        let result = elapsed_periods_since(d("2025-03-31"), d("2025-01-31"), Cadence::Monthly);
        assert_eq!(
            result.unwrap_err(),
            Error::StartPeriodAfterEndPeriod {
                start: "2025-03-31".to_owned(),
                end: "2025-01-31".to_owned()
            }
        );
    }

    #[test]
    fn parse_month_label() {
        let date = parse_period_label_for_cadence("2025-05", Cadence::Monthly).unwrap();
        assert_eq!(date, Date::from_str("2025-05-31").unwrap());
    }

    #[test]
    fn parse_fortnight_label() {
        let date = parse_period_label_for_cadence("2025-02-first-half", Cadence::BiWeekly).unwrap();
        assert_eq!(date, Date::from_str("2025-02-14").unwrap());
    }

    #[test]
    fn parse_label_from_full_date_is_normalized_to_cadence() {
        assert_eq!(
            parse_period_label_for_cadence("2025-05-20", Cadence::Monthly).unwrap(),
            d("2025-05-31")
        );
        assert_eq!(
            parse_period_label_for_cadence("2025-05-20", Cadence::BiWeekly).unwrap(),
            d("2025-05-31")
        );
    }

    #[test]
    fn parse_label_rejects_invalid_shape_or_mismatched_kind() {
        let monthly_fortnight =
            parse_period_label_for_cadence("2025-05-first-half", Cadence::Monthly);
        assert_eq!(
            monthly_fortnight.unwrap_err(),
            Error::CannotExpenseForFortnightWhenCadenceIsMonthly
        );

        let biweekly_month = parse_period_label_for_cadence("2025-05", Cadence::BiWeekly);
        assert_eq!(
            biweekly_month.unwrap_err(),
            Error::CannotExpenseForMonthWhenCadenceIsBiWeekly
        );

        let invalid = parse_period_label_for_cadence("not-a-date", Cadence::Monthly);
        assert!(invalid.is_err());

        let missing_month = parse_period_label_for_cadence("2025", Cadence::Monthly);
        assert!(matches!(
            missing_month,
            Err(Error::FailedToParseDate { .. })
        ));
    }

    #[test]
    fn period_end_from_relative_time_monthly_last_changes_month() {
        let now = period_end_from_relative_time(RelativeTime::current(Granularity::Month)).unwrap();
        let last = period_end_from_relative_time(RelativeTime::last(Granularity::Month)).unwrap();
        assert!(last < now);
    }

    #[test]
    fn period_end_from_relative_time_rejects_unsupported_unit() {
        let relative = RelativeTime::builder()
            .unit(Granularity::Day)
            .amount(0)
            .build();
        let result = period_end_from_relative_time(relative);
        assert!(matches!(result, Err(Error::InvalidPeriod { .. })));
    }

    #[test]
    fn calculate_invoice_number_monthly() {
        let offset = TimestampedInvoiceNumber::builder()
            .offset(100)
            .date(Date::from_str("2025-12-31").unwrap())
            .build();
        let target = Date::from_str("2026-02-28").unwrap();
        let number = calculate_invoice_number(
            &offset,
            &target,
            Cadence::Monthly,
            false,
            &RecordOfPeriodsOff::default(),
        )
        .unwrap();
        assert_eq!(number, InvoiceNumber::from(102));
    }

    #[test]
    fn calculate_invoice_number_subtracts_periods_off_and_adds_expenses_increment() {
        let offset = TimestampedInvoiceNumber::builder()
            .offset(100)
            .date(d("2024-01-31"))
            .build();
        let target = d("2024-04-30");
        let periods_off = RecordOfPeriodsOff::new([d("2024-02-29")]);

        let number =
            calculate_invoice_number(&offset, &target, Cadence::Monthly, true, &periods_off)
                .unwrap();
        assert_eq!(number, InvoiceNumber::from(103));
    }

    #[test]
    fn calculate_invoice_number_rejects_offset_in_periods_off() {
        let offset = TimestampedInvoiceNumber::builder()
            .offset(100)
            .date(d("2025-12-31"))
            .build();
        let periods_off = RecordOfPeriodsOff::new([d("2025-12-31")]);
        let result = calculate_invoice_number(
            &offset,
            &d("2026-01-31"),
            Cadence::Monthly,
            false,
            &periods_off,
        );
        assert_eq!(
            result.unwrap_err(),
            Error::RecordsOffMustNotContainOffsetPeriod {
                offset_period: "2025-12-31".to_owned()
            }
        );
    }

    #[test]
    fn calculate_invoice_number_fails_when_target_is_before_offset() {
        let offset = TimestampedInvoiceNumber::builder()
            .offset(100)
            .date(d("2026-02-28"))
            .build();
        let result = calculate_invoice_number(
            &offset,
            &d("2026-01-31"),
            Cadence::Monthly,
            false,
            &RecordOfPeriodsOff::default(),
        );
        assert_eq!(
            result.unwrap_err(),
            Error::StartPeriodAfterEndPeriod {
                start: "2026-02-28".to_owned(),
                end: "2026-01-31".to_owned()
            }
        );
    }

    #[test]
    fn period_bounds_for_biweekly_first_and_second_half() {
        let (start_first, end_first) = period_bounds(d("2025-05-15"), Cadence::BiWeekly).unwrap();
        assert_eq!(start_first, d("2025-05-01"));
        assert_eq!(end_first, d("2025-05-15"));

        let (start_second, end_second) = period_bounds(d("2025-05-31"), Cadence::BiWeekly).unwrap();
        assert_eq!(start_second, d("2025-05-16"));
        assert_eq!(end_second, d("2025-05-31"));
    }

    #[test]
    fn working_days_between_counts_weekdays_only() {
        let weekends = working_days_between(d("2025-05-17"), d("2025-05-18")).unwrap();
        assert_eq!(*weekends, dec!(0));

        let weekday = working_days_between(d("2025-05-19"), d("2025-05-19")).unwrap();
        assert_eq!(*weekday, dec!(1));
    }

    #[test]
    fn quantity_in_period_biweekly_days_is_less_than_monthly_days() {
        let target = Date::from_str("2025-05-31").unwrap();
        let monthly = quantity_in_period(
            &target,
            Granularity::Day,
            Cadence::Monthly,
            &RecordOfPeriodsOff::default(),
        )
        .unwrap();
        let fortnight = quantity_in_period(
            &target,
            Granularity::Day,
            Cadence::BiWeekly,
            &RecordOfPeriodsOff::default(),
        )
        .unwrap();
        assert!(fortnight < monthly);
    }

    #[test]
    fn quantity_in_period_other_happy_paths() {
        assert_eq!(
            quantity_in_period(
                &d("2025-05-31"),
                Granularity::Month,
                Cadence::Monthly,
                &RecordOfPeriodsOff::default(),
            )
            .unwrap(),
            Quantity::ONE
        );
        assert_eq!(
            quantity_in_period(
                &d("2025-05-31"),
                Granularity::Fortnight,
                Cadence::Monthly,
                &RecordOfPeriodsOff::default(),
            )
            .unwrap(),
            Quantity::TWO
        );
        assert_eq!(
            quantity_in_period(
                &d("2025-05-31"),
                Granularity::Fortnight,
                Cadence::BiWeekly,
                &RecordOfPeriodsOff::default(),
            )
            .unwrap(),
            Quantity::ONE
        );
    }

    #[test]
    fn quantity_in_period_hours_matches_days_times_eight() {
        let target = d("2025-05-31");
        let days = quantity_in_period(
            &target,
            Granularity::Day,
            Cadence::BiWeekly,
            &RecordOfPeriodsOff::default(),
        )
        .unwrap();
        let hours = quantity_in_period(
            &target,
            Granularity::Hour,
            Cadence::BiWeekly,
            &RecordOfPeriodsOff::default(),
        )
        .unwrap();
        assert_eq!(hours, Quantity::EIGHT.mul(*days));
    }

    #[test]
    fn quantity_in_period_fails_when_target_is_in_periods_off() {
        let target = d("2025-05-31");
        let periods_off = RecordOfPeriodsOff::new([target]);
        let result = quantity_in_period(&target, Granularity::Day, Cadence::Monthly, &periods_off);
        assert_eq!(
            result.unwrap_err(),
            Error::TargetPeriodMustNotBeInRecordOfPeriodsOff {
                target_period: "2025-05-31".to_owned()
            }
        );
    }

    #[test]
    fn quantity_in_period_fails_for_biweekly_month_granularity() {
        let target = Date::sample();
        let result = quantity_in_period(
            &target,
            Granularity::Month,
            Cadence::BiWeekly,
            &RecordOfPeriodsOff::default(),
        );
        assert_eq!(
            result.unwrap_err(),
            Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly
        );
    }
}
