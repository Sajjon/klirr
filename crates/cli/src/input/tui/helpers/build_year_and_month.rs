use inquire::{CustomType, error::InquireResult};

use crate::prelude::*;

pub fn build_year_month_inner(
    help: impl Into<Option<String>>,
    default_year: Option<&Year>,
    default_month: Option<&Month>,
) -> InquireResult<Option<YearAndMonth>> {
    let default = YearAndMonth::last();
    let default_year = default_year.unwrap_or(default.year());
    let default_month = default_month.unwrap_or(default.month());

    let help_message = format_help_skippable(help);

    let Some(year) = CustomType::<Year>::new("Year?")
        .with_help_message(&help_message)
        .with_default(*default_year)
        .prompt_skippable()?
    else {
        return Ok(None);
    };

    let Some(month) = CustomType::<Month>::new("Month?")
        .with_help_message(&help_message)
        .with_default(*default_month)
        .prompt_skippable()?
    else {
        return Ok(None);
    };

    Ok(Some(
        YearAndMonth::builder().year(year).month(month).build(),
    ))
}
