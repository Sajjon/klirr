use crate::{Date, Error, Result};
use inquire::{DateSelect, error::InquireResult};

#[allow(unused)]
pub fn build_date(prompt: Option<String>) -> Result<Date> {
    fn inner(prompt: Option<String>) -> InquireResult<Date> {
        let date = DateSelect::new(&prompt.unwrap_or("Date?".to_owned()))
            .with_default(chrono::NaiveDate::from_ymd_opt(2021, 8, 1).unwrap())
            .with_min_date(chrono::NaiveDate::from_ymd_opt(2021, 8, 1).unwrap())
            .with_max_date(chrono::NaiveDate::from_ymd_opt(2021, 12, 31).unwrap())
            .with_week_start(chrono::Weekday::Mon)
            .prompt()?;

        Ok(Date::from(date))
    }
    inner(prompt).map_err(|e| Error::InvalidDate {
        underlying: e.to_string(),
    })
}
