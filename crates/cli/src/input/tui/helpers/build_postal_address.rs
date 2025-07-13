use inquire::{Text, error::InquireResult};

use crate::prelude::*;

pub fn build_postal_address(
    owner: impl AsRef<str>,
    default: &PostalAddress,
) -> InquireResult<PostalAddress> {
    let text = |part: &str| format!("{}'s {part} [postal address]?", owner.as_ref());

    let zip = Text::new(&text("ZIP code"))
        .with_default(default.zip())
        .prompt()?;

    let city = Text::new(&text("City"))
        .with_default(default.city())
        .prompt()?;

    let country = Text::new(&text("Country"))
        .with_default(default.country())
        .prompt()?;

    let street_line1 = Text::new(&text("Street Line 1"))
        .with_default(default.street_address().line_1())
        .prompt()?;
    let street_line2 = Text::new(&text("Street Line 2"))
        .with_default(default.street_address().line_2())
        .with_help_message(&format_help_skippable(
            "e.g. C/o or Apartment 12".to_owned(),
        ))
        .prompt_skippable()?
        .unwrap_or("".to_owned());

    let street_address = StreetAddress::builder()
        .line_1(street_line1)
        .line_2(street_line2)
        .build();

    let address = default
        .clone()
        .with_street_address(street_address)
        .with_zip(zip)
        .with_country(country)
        .with_city(city);

    Ok(address)
}
