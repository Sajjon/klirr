use inquire::{CustomType, error::InquireResult};

use crate::{
    Cadence, Date, Day, Month, MonthHalf, WithOptionalDefault, WithPossibleValues,
    build_year_month_inner,
};

fn first_half_end_day(month: Month) -> Day {
    if month.is_february() {
        Day::try_from(14).expect("valid day")
    } else {
        Day::try_from(15).expect("valid day")
    }
}

pub fn build_period(
    help: impl Into<Option<String>>,
    default: Option<Date>,
    cadence: Cadence,
) -> InquireResult<Option<Date>> {
    let help = help.into();

    let Some((year, month)) = build_year_month_inner(
        help.clone(),
        default.as_ref().map(|d| d.year()),
        default.as_ref().map(|d| d.month()),
    )?
    else {
        return Ok(None);
    };
    match cadence {
        Cadence::Monthly => Ok(Some(
            Date::builder()
                .year(year)
                .month(month)
                .day(month.last_day(year))
                .build(),
        )),
        Cadence::BiWeekly => {
            let half = CustomType::<MonthHalf>::new("Half of month?")
                .with_help_possible_values()
                .with_optional_default(&default.and_then(|d| {
                    if d.month() == &month {
                        Some(MonthHalf::from(d))
                    } else {
                        None
                    }
                }))
                .prompt()?;

            let day = match half {
                MonthHalf::First => first_half_end_day(month),
                MonthHalf::Second => month.last_day(year),
            };
            Ok(Some(
                Date::builder().year(year).month(month).day(day).build(),
            ))
        }
    }
}
