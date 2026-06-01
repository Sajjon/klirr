use inquire::{Confirm, CustomType, Text, error::InquireResult};

use crate::{
    Cadence, Granularity, InvoiceDataFromTuiError, Rate, Result, ServiceFees, UnitPrice,
    WithPossibleValues,
};

pub fn build_service_fees(default: &ServiceFees) -> Result<ServiceFees> {
    fn inner(default: &ServiceFees) -> InquireResult<ServiceFees> {
        let text = |part: &str| format!("Service {part}?");
        let name = Text::new(&text("Name"))
            .with_default(default.name())
            .prompt()?;

        let cadence = CustomType::<Cadence>::new("How often do you invoice?")
            .with_help_possible_values()
            .with_default(*default.cadence())
            .prompt()?;

        let granularity = CustomType::<Granularity>::new("Do you invoice per month, day or hour? Next question will be the rate which is per time unit you provide here")
          .with_help_possible_values()
            .with_default(default.rate().granularity())
            .prompt()?;

        let unit_price = CustomType::<UnitPrice>::new("Unit price (excl. VAT)?")
            .with_help_message(&format!(
                "Price per {}, excluding VAT (VAT is configured separately on payment info), e.g. {}",
                granularity,
                granularity.example_rate()
            ))
            .with_default(default.unit_price())
            .prompt()?;

        let rate = Rate::from((unit_price, granularity));

        let off_on_bank_holidays = Confirm::new("Off on bank holidays?")
            .with_help_message(
                "If yes, public holidays in the vendor's country are deducted from billable \
                 working days (day/hour rates only). Looked up online and cached.",
            )
            .with_default(*default.off_on_bank_holidays())
            .prompt()?;

        Ok(ServiceFees::builder()
            .name(name)
            .cadence(cadence)
            .rate(rate)
            .off_on_bank_holidays(off_on_bank_holidays)
            .build()
            .unwrap())
    }
    inner(default)
        .map_err(InvoiceDataFromTuiError::invalid_service_fees)
        .map_err(crate::Error::from)
}
