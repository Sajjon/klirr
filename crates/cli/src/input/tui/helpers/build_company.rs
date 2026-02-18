use inquire::{Text, error::InquireResult};

use crate::{
    CompanyInformation, Error, Result, WithOptionalDefault, build_postal_address,
    format_help_skippable,
};

pub fn build_company(
    owner: impl AsRef<str>,
    default: &CompanyInformation,
) -> Result<CompanyInformation> {
    fn inner(owner: String, default: &CompanyInformation) -> InquireResult<CompanyInformation> {
        let text = |part: &str| format!("{owner}'s {part}?");
        let name = Text::new(&text("name"))
            .with_default(default.company_name())
            .prompt()?;

        let org_no = Text::new(&text("organisation number"))
            .with_default(default.organisation_number())
            .prompt()?;

        let vat = Text::new(&text("VAT number"))
            .with_default(default.vat_number())
            .prompt()?;

        let contact_person = Text::new(&text("contact person"))
            .with_optional_default(default.contact_person())
            .with_help_message(&format_help_skippable(
                if owner.to_lowercase().contains("client") {
                    "Your reference".to_owned()
                } else {
                    "Our reference".to_owned()
                },
            ))
            .prompt_skippable()?;

        let postal_address = build_postal_address(&owner, default.postal_address())?;

        let company_info = default
            .clone()
            .with_company_name(name)
            .with_contact_person(contact_person)
            .with_organisation_number(org_no)
            .with_postal_address(postal_address)
            .with_vat_number(vat);

        Ok(company_info)
    }
    inner(owner.as_ref().to_owned(), default).map_err(|e| Error::InvalidCompanyInformation {
        reason: format!("{:?}", e),
    })
}
