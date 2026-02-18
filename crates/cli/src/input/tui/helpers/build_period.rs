use inquire::{CustomType, error::InquireResult};

use crate::{
    Cadence, IsPeriod, MonthHalf, PeriodAnno, WithOptionalDefault, WithPossibleValues,
    YearMonthAndFortnight, build_year_month_inner,
};

pub fn build_period(
    help: impl Into<Option<String>>,
    default: Option<PeriodAnno>,
    cadence: Cadence,
) -> InquireResult<Option<PeriodAnno>> {
    let help = help.into();

    let Some(ym) = build_year_month_inner(
        help.clone(),
        default.as_ref().map(|d| d.year()),
        default.as_ref().map(|d| d.month()),
    )?
    else {
        return Ok(None);
    };
    match cadence {
        Cadence::Monthly => Ok(Some(ym.into())),
        Cadence::BiWeekly => {
            let half = CustomType::<MonthHalf>::new("Half of month?")
                .with_help_possible_values()
                .with_optional_default(&default.and_then(|d| {
                    d.try_unwrap_year_month_and_fortnight()
                        .ok()
                        .map(|yam| *yam.half())
                }))
                .prompt()?;

            Ok(Some(
                YearMonthAndFortnight::builder()
                    .year(*ym.year())
                    .month(*ym.month())
                    .half(half)
                    .build()
                    .into(),
            ))
        }
    }
}
