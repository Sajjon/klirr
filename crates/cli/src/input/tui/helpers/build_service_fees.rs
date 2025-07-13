use inquire::{CustomType, Text, error::InquireResult};

use crate::prelude::*;

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

        let unit_price = CustomType::<UnitPrice>::new("Unit price?")
            .with_help_message(&format!(
                "Price per {}, e.g. {}",
                granularity,
                granularity.example_rate()
            ))
            .with_default(default.unit_price())
            .prompt()?;

        let rate = Rate::from((unit_price, granularity));

        Ok(ServiceFees::builder()
            .name(name)
            .cadence(cadence)
            .rate(rate)
            .build()
            .unwrap())
    }
    inner(default).map_err(|e| Error::InvalidServiceFees {
        reason: format!("{:?}", e),
    })
}
