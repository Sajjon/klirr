use crate::{
    Cadence, Date, Error, Granularity, InvoiceNumber, Quantity, RecordOfPeriodsOff, RelativeTime,
    Result, TimestampedInvoiceNumber,
};
use klirr_foundation::{
    CalendarError, calculate_period_number, normalize_period_end_date_for_cadence as normalize,
    parse_period_label_for_cadence as parse_period_label,
    period_end_from_relative_time as from_relative, quantity_in_period as quantity_in_period_inner,
};

fn map_calendar_error(error: CalendarError) -> Error {
    match error {
        CalendarError::Model(model_error) => Error::from(model_error),
        CalendarError::InvalidPeriod { bad_value } => Error::InvalidPeriod { bad_value },
        CalendarError::StartPeriodAfterEndPeriod { start, end } => {
            Error::StartPeriodAfterEndPeriod { start, end }
        }
        CalendarError::RecordsOffMustNotContainOffsetPeriod { offset_period } => {
            Error::RecordsOffMustNotContainOffsetPeriod { offset_period }
        }
        CalendarError::TargetPeriodMustNotBeInRecordOfPeriodsOff { target_period } => {
            Error::TargetPeriodMustNotBeInRecordOfPeriodsOff { target_period }
        }
        CalendarError::CannotInvoiceForMonthWhenCadenceIsBiWeekly => {
            Error::CannotInvoiceForMonthWhenCadenceIsBiWeekly
        }
        CalendarError::CannotExpenseForMonthWhenCadenceIsBiWeekly => {
            Error::CannotExpenseForMonthWhenCadenceIsBiWeekly
        }
        CalendarError::CannotExpenseForFortnightWhenCadenceIsMonthly => {
            Error::CannotExpenseForFortnightWhenCadenceIsMonthly
        }
        CalendarError::InvalidDate { underlying } => Error::InvalidDate { underlying },
    }
}

/// Normalizes a date to the cadence-aligned period-end date.
///
/// # Examples
/// ```
/// extern crate klirr_core_invoice;
/// use klirr_core_invoice::*;
///
/// let normalized = normalize_period_end_date_for_cadence(
///     "2025-05-30".parse::<Date>().unwrap(),
///     Cadence::Monthly,
/// )
/// .unwrap();
/// assert_eq!(normalized.to_string(), "2025-05-31");
/// ```
pub fn normalize_period_end_date_for_cadence(date: Date, cadence: Cadence) -> Result<Date> {
    normalize(date, cadence).map_err(map_calendar_error)
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
    from_relative(relative).map_err(map_calendar_error)
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
/// ```
pub fn parse_period_label_for_cadence(value: &str, cadence: Cadence) -> Result<Date> {
    parse_period_label(value, cadence).map_err(map_calendar_error)
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
    calculate_period_number(
        **offset.offset(),
        offset.date(),
        target_date,
        cadence,
        is_expenses,
        record_of_periods_off,
    )
    .map(InvoiceNumber::from)
    .map_err(map_calendar_error)
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
    quantity_in_period_inner(target_date, granularity, cadence, record_of_periods_off)
        .map_err(map_calendar_error)
}
