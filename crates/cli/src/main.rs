mod init_logging;
mod input;
mod run_invoice;

pub mod prelude {
    pub(crate) use clap::{Parser, Subcommand};
    pub(crate) use derive_more::FromStr;
    pub(crate) use invoice_typst_logic::prelude::*;

    pub(crate) use crate::input::*;
}

use core::str;

use inquire::{CustomType, DateSelect, Text, error::InquireResult};
use prelude::*;

const HOW_TO_SKIP_INSTRUCTION: &str = "Skip with ESC";

fn build_postal_address(owner: impl AsRef<str>) -> InquireResult<PostalAddress> {
    let sample = PostalAddress::sample();
    let text = |part: &str| format!("{}'s {part} [postal address]?", owner.as_ref());

    let street_line1 = Text::new(&text("Street Line 1"))
        .with_default(sample.street_address().line_1())
        .prompt()?;
    let street_line2 = Text::new(&text("Street Line 2"))
        .with_default(sample.street_address().line_2())
        .prompt_skippable()?
        .unwrap_or("".to_owned());

    let street_address = StreetAddress::builder()
        .line_1(street_line1)
        .line_2(street_line2)
        .build();

    let zip = Text::new(&text("ZIP code"))
        .with_default(sample.zip())
        .prompt()?;

    let city = Text::new(&text("City"))
        .with_default(sample.city())
        .prompt()?;

    let country = Text::new(&text("Country"))
        .with_default(sample.country())
        .prompt()?;

    let address = sample
        .with_street_address(street_address)
        .with_zip(zip)
        .with_country(country)
        .with_city(city);

    Ok(address)
}

fn build_company(owner: impl AsRef<str>) -> Result<CompanyInformation> {
    fn inner(owner: String) -> InquireResult<CompanyInformation> {
        let sample = CompanyInformation::sample();
        let text = |part: &str| format!("{owner}'s {part}?");
        let name = Text::new(&text("name"))
            .with_default(sample.company_name())
            .prompt()?;

        let org_no = Text::new(&text("organisation number"))
            .with_default(sample.organisation_number())
            .prompt()?;

        let vat = Text::new(&text("VAT number"))
            .with_default(sample.vat_number())
            .prompt()?;

        let contact_person = Text::new(&text("contact person"))
            .with_default(
                sample
                    .contact_person()
                    .as_ref()
                    .expect("Expected contact person to be set"),
            )
            .prompt_skippable()?;

        let postal_address = build_postal_address(&owner)?;

        let vendor = sample
            .with_company_name(name)
            .with_contact_person(contact_person)
            .with_organisation_number(org_no)
            .with_postal_address(postal_address)
            .with_vat_number(vat);

        Ok(vendor)
    }
    inner(owner.as_ref().to_owned()).map_err(|e| Error::InvalidCompanyInformation {
        reason: format!("{:?}", e),
    })
}

fn build_date(prompt: Option<String>) -> Result<Date> {
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

fn format_help_message(help: Option<String>) -> String {
    help.map_or_else(
        || HOW_TO_SKIP_INSTRUCTION.to_owned(),
        |h| format!("{HOW_TO_SKIP_INSTRUCTION}: {h}"),
    )
}

fn build_year_month_inner(help: impl Into<Option<String>>) -> InquireResult<Option<YearAndMonth>> {
    let help = help.into();
    let current = YearAndMonth::current();

    let help_message = format_help_message(help);

    let Some(year) = CustomType::<Year>::new("Year?")
        .with_help_message(&help_message)
        .with_default(*current.year())
        .prompt_skippable()?
    else {
        return Ok(None);
    };

    let Some(month) = CustomType::<Month>::new("Month?")
        .with_help_message(&help_message)
        .with_default(*current.month())
        .prompt_skippable()?
    else {
        return Ok(None);
    };

    Ok(Some(
        YearAndMonth::builder().year(year).month(month).build(),
    ))
}

fn build_year_month(help: impl Into<Option<String>>) -> Result<Option<YearAndMonth>> {
    build_year_month_inner(help).map_err(|e| Error::InvalidYearAndMonth {
        underlying: e.to_string(),
    })
}

fn build_invoice_info() -> Result<ProtoInvoiceInfo> {
    fn inner() -> InquireResult<ProtoInvoiceInfo> {
        let sample = ProtoInvoiceInfo::sample();
        let invoice_number_offset = CustomType::<InvoiceNumber>::new("What is the last invoice number you issued? We call this the 'offset'")
        .with_help_message("Next you will be asked about which year and month that last invoice was issued, together, these two values will be used to calculate all future invoice numbers, by calculating the number of elapsed months since that offset month and adding it to the offset number. Skip if you don't have an offset number yet.")
        .with_default(InvoiceNumber::default())
        .prompt_skippable()?.unwrap_or_default();

        let invoice_number_offset_month = build_year_month_inner(
            "When was that invoice issued? (Used to calculate future invoice numbers)".to_owned(),
        )?
        // if we use `0` as offset and set month to last month, then the next invoice number will be `1` for this month, which is correct.
        .unwrap_or(YearAndMonth::last());

        let offset = TimestampedInvoiceNumber::builder()
            .offset(invoice_number_offset)
            .month(invoice_number_offset_month)
            .build();

        todo!()
    }
    inner().map_err(|e| Error::InvalidInvoiceInfo {
        reason: format!("{:?}", e),
    })
}

fn init_data_directory(input: &DataInitInput) -> Result<()> {
    info!(
        "Initializing data directory at: {}",
        input.data_dir().display()
    );
    // Here you would implement the logic to initialize the data directory.
    // For now, we just log the action.
    let vendor = build_company("Your company")?;
    info!("Vendor information: {:#?}", vendor);
    let client = build_company("Your client")?;
    info!("Client information: {:#?}", client);
    let invoice_info = build_invoice_info()?;
    info!("Invoice info: {:#?}", invoice_info);
    Ok(())
}

fn validate_data_directory(input: &DataValidateInput) -> Result<()> {
    info!(
        "Validating data directory at: {}",
        input.data_dir().display()
    );
    // Here you would implement the logic to validate the data directory.
    // For now, we just log the action.
    Ok(())
}

fn record_month_off(input: &MonthOffInput) -> Result<()> {
    info!("Recording month off for: {}", input.month());
    todo!("implement me")
}

fn record_expenses(input: &ExpensesInput) -> Result<()> {
    info!("Recording expenses for: {}", input.month());
    todo!("implement me")
}

fn run_data_admin(input: input::DataAdminInput) -> Result<()> {
    match input.command() {
        input::DataAdminInputCommands::Init(init_input) => init_data_directory(init_input),
        input::DataAdminInputCommands::Validate(validate_input) => {
            validate_data_directory(validate_input)
        }
        input::DataAdminInputCommands::MonthOff(month_off_input) => {
            record_month_off(month_off_input)
        }
        input::DataAdminInputCommands::Expenses(expenses_input) => record_expenses(expenses_input),
    }
}

fn run(input: input::CliArgs) -> Result<()> {
    match input.command {
        input::Commands::Invoice(invoice_input) => run_invoice::run(invoice_input),
        input::Commands::Data(data_admin_input) => run_data_admin(data_admin_input),
    }
}

fn main() {
    init_logging::init_logging();
    // let invoice_number_offset_month = build_year_month_inner(
    //     // "When was that invoice issued? (Used to calculate future invoice numbers)".to_owned(),
    //     None,
    // )
    // .unwrap();
    // info!(
    //     "Selected year and month: {:#?}",
    //     invoice_number_offset_month
    // );
    let input = input::CliArgs::parse();
    let _ = run(input).inspect_err(|e| error!("Error creating PDF: {}", e));
}
