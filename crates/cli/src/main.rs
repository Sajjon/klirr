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

use inquire::{Text, error::InquireResult};
use prelude::*;

fn build_postal_address(owner: String) -> InquireResult<PostalAddress> {
    let sample = PostalAddress::sample();
    let text = |part: &str| format!("Postal Address of {} > {}", owner, part);

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

fn build_vendor() -> Result<CompanyInformation> {
    fn inner() -> InquireResult<CompanyInformation> {
        let vendor = CompanyInformation::sample();

        let name = Text::new("What is the name of your company?")
            .with_default(vendor.company_name())
            .with_help_message("This will be used as the name of the vendor on the invoice.")
            .prompt()?;

        let org_no = Text::new("Your company's organization number?")
            .with_default(vendor.organisation_number())
            .with_help_message(
                "Org number will be shown next to your payment details on the invoice.",
            )
            .prompt()?;

        let vat = Text::new("Your company's VAT?")
            .with_default(vendor.vat_number())
            .with_help_message(
                "VAT number will be shown next to your payment details on the invoice.",
            )
            .prompt()?;

        let contact_person = Text::new("Who is the contact person at your company?")
            .with_default(
                vendor
                    .contact_person()
                    .as_ref()
                    .expect("Expected contact person for sample vendor to be set"),
            )
            .with_help_message("This will be used as 'Our reference' on the invoice.")
            .prompt_skippable()?;

        let postal_address = build_postal_address("Your company".to_owned())?;

        let vendor = vendor
            .with_company_name(name)
            .with_contact_person(contact_person)
            .with_organisation_number(org_no)
            .with_postal_address(postal_address)
            .with_vat_number(vat);

        Ok(vendor)
    }
    inner().map_err(|e| Error::InvalidCompanyInformation {
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
    let vendor = build_vendor()?;
    info!("Vendor information: {:#?}", vendor);
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

fn run_data_admin(input: input::DataAdminInput) -> Result<()> {
    match input.command() {
        input::DataAdminInputCommands::Init(init_input) => init_data_directory(init_input),
        input::DataAdminInputCommands::Validate(validate_input) => {
            validate_data_directory(validate_input)
        }
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
    let input = input::CliArgs::parse();
    let _ = run(input).inspect_err(|e| error!("Error creating PDF: {}", e));
}
