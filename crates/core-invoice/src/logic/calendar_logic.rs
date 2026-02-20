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

/// Normalizes any date/label-parsed date to the cadence-aligned period-end date.
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
pub fn period_end_from_relative_time(relative: RelativeTime) -> Result<Date> {
    let current_period_end = period_end_for_unit(Date::today(), *relative.unit())?;
    shift_period_end(current_period_end, *relative.unit(), *relative.amount())
}

fn parse_legacy_period_label(value: &str) -> Result<(Date, Granularity)> {
    let mut parts = value.splitn(3, '-');
    let Some(year_part) = parts.next() else {
        return Err(Error::InvalidPeriod {
            bad_value: value.to_owned(),
        });
    };
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

    if granularity > cadence.max_granularity() {
        return Err(Error::GranularityTooCoarse {
            granularity,
            max_granularity: cadence.max_granularity(),
            target_period: target_date.to_string(),
        });
    }

    match granularity {
        Granularity::Month => match cadence {
            Cadence::Monthly => Ok(Quantity::ONE),
            Cadence::BiWeekly => Err(Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly),
        },
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
    use std::str::FromStr;

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
    fn period_end_from_relative_time_monthly_last_changes_month() {
        let now = period_end_from_relative_time(RelativeTime::current(Granularity::Month)).unwrap();
        let last = period_end_from_relative_time(RelativeTime::last(Granularity::Month)).unwrap();
        assert!(last < now);
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
