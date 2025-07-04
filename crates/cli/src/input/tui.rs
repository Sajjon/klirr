use crate::prelude::*;
use inquire::{
    Confirm, CustomType, DateSelect, InquireError, Password, PasswordDisplayMode, Text,
    error::InquireResult,
    set_global_render_config,
    ui::{RenderConfig, StyleSheet},
};

const HOW_TO_SKIP_INSTRUCTION: &str = "Skip with ESC";

fn build_postal_address(
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

trait WithOptionalDefault<'o, T> {
    fn with_optional_default(self, default: &'o Option<T>) -> Self;
}
impl<'a, 'o: 'a, T: AsRef<str>> WithOptionalDefault<'o, T> for Text<'a> {
    fn with_optional_default(self, default: &'o Option<T>) -> Self {
        match default {
            Some(value) => self.with_default(value.as_ref()),
            None => self,
        }
    }
}
impl<'a, 'o: 'a, T: Clone> WithOptionalDefault<'o, T> for CustomType<'a, T> {
    fn with_optional_default(self, default: &'o Option<T>) -> Self {
        match default {
            Some(value) => self.with_default(value.clone()),
            None => self,
        }
    }
}

fn build_company(
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

#[allow(unused)]
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

fn format_help_skippable(help: impl Into<Option<String>>) -> String {
    help.into().map_or_else(
        || HOW_TO_SKIP_INSTRUCTION.to_owned(),
        |h| format!("{HOW_TO_SKIP_INSTRUCTION}: {h}"),
    )
}

fn build_year_month_inner(
    help: impl Into<Option<String>>,
    default: Option<YearAndMonth>,
) -> InquireResult<Option<YearAndMonth>> {
    let default_value = default.unwrap_or(YearAndMonth::last());

    let help_message = format_help_skippable(help);

    let Some(year) = CustomType::<Year>::new("Year?")
        .with_help_message(&help_message)
        .with_default(*default_value.year())
        .prompt_skippable()?
    else {
        return Ok(None);
    };

    let Some(month) = CustomType::<Month>::new("Month?")
        .with_help_message(&help_message)
        .with_default(*default_value.month())
        .prompt_skippable()?
    else {
        return Ok(None);
    };

    Ok(Some(
        YearAndMonth::builder().year(year).month(month).build(),
    ))
}

#[allow(unused)]
fn build_year_month(
    help: impl Into<Option<String>>,
    default: Option<YearAndMonth>,
) -> Result<Option<YearAndMonth>> {
    build_year_month_inner(help, default).map_err(|e| Error::InvalidYearAndMonth {
        underlying: e.to_string(),
    })
}

fn build_invoice_info(default: &ProtoInvoiceInfo) -> Result<ProtoInvoiceInfo> {
    fn inner(default: &ProtoInvoiceInfo) -> InquireResult<ProtoInvoiceInfo> {
        let invoice_number_offset = CustomType::<InvoiceNumber>::new(
            "What is the last invoice number you issued? We call this the 'offset'",
        )
        .with_help_message(&format_help_skippable(
            "Used with the date of that invoice to calculate future invoice numbers.".to_owned(),
        ))
        .with_default(default.offset().offset().clone())
        .prompt_skippable()?
        .unwrap_or_default();

        let invoice_number_offset_month = build_year_month_inner(
            "When was that invoice issued? (Used to calculate future invoice numbers)".to_owned(),
            Some(*default.offset().month()),
        )?
        // if we use `0` as offset and set month to last month, then the next invoice number will be `1` for this month, which is correct.
        .unwrap_or(YearAndMonth::last());

        let offset = TimestampedInvoiceNumber::builder()
            .offset(invoice_number_offset)
            .month(invoice_number_offset_month)
            .build();

        let purchase_order = CustomType::<PurchaseOrder>::new("Purchase order number (optional)")
            .with_optional_default(default.purchase_order())
            .with_help_message(&format_help_skippable(
                "If you have a purchase order number, enter it here".to_owned(),
            ))
            .prompt_skippable()?;

        let footer_text = CustomType::<FooterText>::new("Footer text (optional)")
            .with_help_message(&format_help_skippable(
                "This is shown in the bottom of the invoice, it can e.g. be 'Reverse Charge'"
                    .to_owned(),
            ))
            .with_default(FooterText::default())
            .prompt_skippable()?;

        let emphasize_color_hex = CustomType::<HexColor>::new("Emphasize color (optional)")
            .with_optional_default(default.emphasize_color_hex())
            .with_help_message(&format_help_skippable(
                "This is used to emphasize certain parts of the invoice, e.g. '#e6007a'".to_owned(),
            ))
            .prompt_skippable()?;

        let info = ProtoInvoiceInfo::builder()
            .offset(offset)
            .purchase_order(purchase_order)
            .footer_text(footer_text)
            .emphasize_color_hex(emphasize_color_hex)
            .months_off_record(default.months_off_record().clone())
            .build();

        Ok(info)
    }
    inner(default).map_err(|e| Error::InvalidInvoiceInfo {
        reason: format!("{:?}", e),
    })
}

fn build_payment_info(default: &PaymentInformation) -> Result<PaymentInformation> {
    fn inner(default: &PaymentInformation) -> InquireResult<PaymentInformation> {
        let text = |part: &str| format!("Payment {part}?");
        let bank_name = Text::new(&text("Bank Name"))
            .with_default(default.bank_name())
            .prompt()?;
        let iban = Text::new(&text("IBAN"))
            .with_default(default.iban())
            .prompt()?;

        let bic = Text::new(&text("BIC"))
            .with_default(default.bic())
            .prompt()?;

        let currency = CustomType::<Currency>::new("Currency?")
            .with_help_message("The currency you want to use for the invoice, e.g. 'EUR'")
            .with_default(*default.currency())
            .prompt()?;

        let payment_terms = CustomType::<PaymentTerms>::new("Payment terms?")
            .with_help_message("The payment terms for this invoice, e.g. 'Net 30'")
            .with_default(PaymentTerms::net30())
            .prompt()?;

        let payment_info = default
            .clone()
            .with_bank_name(bank_name)
            .with_iban(iban)
            .with_bic(bic)
            .with_currency(currency)
            .with_terms(payment_terms);

        Ok(payment_info)
    }
    inner(default).map_err(|e| Error::InvalidPaymentInfo {
        reason: format!("{:?}", e),
    })
}

fn build_service_fees(default: &ServiceFees) -> Result<ServiceFees> {
    fn inner(default: &ServiceFees) -> InquireResult<ServiceFees> {
        let text = |part: &str| format!("Service {part}?");
        let name = Text::new(&text("Name"))
            .with_default(default.name())
            .prompt()?;

        let unit_price = CustomType::<UnitPrice>::new("Unit price?")
            .with_help_message("The price per day, e.g. '1000'")
            .with_default(*default.unit_price())
            .prompt()?;

        let service_fees = default.clone().with_name(name).with_unit_price(unit_price);

        Ok(service_fees)
    }
    inner(default).map_err(|e| Error::InvalidServiceFees {
        reason: format!("{:?}", e),
    })
}

#[derive(Display, Clone, Copy, Debug)]
enum EmailAddressRole {
    #[display("Reply-To")]
    ReplyTo,
    #[display("Sender")]
    Sender,
    #[display("Recipient")]
    Recipient,
    #[display("CC")]
    Cc,
    #[display("BCC")]
    Bcc,
}

fn ask_for_email_address_skippable(role: EmailAddressRole) -> Result<Option<EmailAddress>> {
    let email = Text::new(&format!("{}'s email address?", role))
        .with_help_message(&format_help_skippable(format!(
            "Email address for {}",
            role
        )))
        .prompt_skippable()
        .map_err(|e| Error::InvalidEmailAddress {
            role: role.to_string(),
            underlying: e.to_string(),
        })?;
    let Some(email) = email else {
        return Ok(None);
    };
    EmailAddress::from_str(&email)
        .map_err(|e| Error::InvalidEmailAddress {
            role: role.to_string(),
            underlying: e.to_string(),
        })
        .map(Some)
}
fn ask_for_email_address(role: EmailAddressRole) -> Result<EmailAddress> {
    Text::new(&format!("{}'s email address?", role))
        .with_help_message(&format!("Email address for {}", role))
        .prompt()
        .and_then(|s| EmailAddress::from_str(&s).map_err(|e| InquireError::Custom(e.into())))
        .map_err(|e| Error::InvalidEmailAddress {
            role: role.to_string(),
            underlying: e.to_string(),
        })
}

fn ask_for_many_email_addresses(role: EmailAddressRole) -> Result<IndexSet<EmailAddress>> {
    let mut emails = IndexSet::new();
    loop {
        let Some(email) = ask_for_email_address_skippable(role)? else {
            break;
        };
        if emails.contains(&email) {
            warn!("Email address already exists, skipping");
            continue;
        }
        emails.insert(email);
        let another = Confirm::new(&format!("Add another {} email address?", role))
            .with_default(true)
            .prompt()
            .unwrap_or(true);
        if !another {
            break;
        }
    }
    Ok(emails)
}

fn ask_for_email_account(role: EmailAddressRole) -> Result<EmailAccount> {
    let name = Text::new(&format!("Email account {} name?", role))
        .with_help_message(&format!("Will show up as the {} name", role))
        .prompt()
        .map_err(|e| Error::InvalidNameForEmail {
            role: role.to_string(),
            underlying: e.to_string(),
        })?;
    let email = ask_for_email_address(role)?;
    Ok(EmailAccount::builder().name(name).email(email).build())
}

fn ask_for_email_account_skippable(role: EmailAddressRole) -> Result<Option<EmailAccount>> {
    let name = Text::new(&format!("Email account {} name?", role))
        .with_help_message(&format_help_skippable(format!(
            "Will show up as the {} name",
            role
        )))
        .prompt_skippable()
        .map_err(|e| Error::InvalidNameForEmail {
            role: role.to_string(),
            underlying: e.to_string(),
        })?;
    let Some(name) = name else { return Ok(None) };
    let Some(email) = ask_for_email_address_skippable(role)? else {
        return Ok(None);
    };
    Ok(Some(
        EmailAccount::builder().name(name).email(email).build(),
    ))
}

use inquire::validator::Validation;

fn ask_for_password(with_confirmation: bool, prompt: &str, help: &str) -> Result<String> {
    // We use 4 as character minimum length for SMTP app passwords
    // and encryption passwords.
    let validator = |input: &str| {
        let min_length = 4;
        if input.chars().count() < min_length {
            Ok(Validation::Invalid(
                format!("Password must have at least {} characters.", min_length).into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let help_message = format!("{} (Press CTRL+R to reveal)", help);

    let mut inquire = Password::new(prompt)
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_help_message(&help_message)
        .with_validator(validator);

    if !with_confirmation {
        inquire = inquire.without_confirmation();
    }

    inquire
        .prompt()
        .map_err(|e| Error::InvalidPasswordForEmail {
            purpose: prompt.to_owned(),
            underlying: e.to_string(),
        })
}

fn ask_for_email_encryption_password_with_confirmation(with_confirmation: bool) -> Result<String> {
    ask_for_password(
        with_confirmation,
        "SMTP App Password Encryption",
        "Used to encrypt the SMTP app password",
    )
}

pub fn ask_for_email_encryption_password() -> Result<String> {
    ask_for_email_encryption_password_with_confirmation(false)
}

pub fn ask_for_email() -> Result<EncryptedEmailSettings> {
    config_render();

    let smtp_app_password = ask_for_password(true, "SMTP App Password", "Used to send email")?;
    let smtp_app_password_encryption_password =
        ask_for_email_encryption_password_with_confirmation(true)?;
    let smtp_server = CustomType::<SmtpServer>::new("SMTP server?")
        .with_help_message("The SMTP server to use for sending emails")
        .with_default(SmtpServer::default())
        .prompt()
        .map_err(|e| Error::InvalidSmtpServer {
            underlying: e.to_string(),
        })?;
    let sender = ask_for_email_account(EmailAddressRole::Sender)?;
    let reply_to = ask_for_email_account_skippable(EmailAddressRole::ReplyTo)?;
    let recipients = ask_for_many_email_addresses(EmailAddressRole::Recipient)?;
    if recipients.is_empty() {
        return Err(Error::RecipientAddressesCannotBeEmpty);
    }
    let cc_recipients = ask_for_many_email_addresses(EmailAddressRole::Cc)?;
    let bcc_recipients = ask_for_many_email_addresses(EmailAddressRole::Bcc)?;

    let encrypted_smtp_app_password = EncryptedAppPassword::new_by_deriving_and_encrypting(
        smtp_app_password,
        smtp_app_password_encryption_password,
    );
    let email_settings = EncryptedEmailSettings::builder()
        .sender(sender)
        .smtp_server(smtp_server)
        .smtp_app_password(encrypted_smtp_app_password)
        .reply_to(reply_to)
        .public_recipients(recipients.clone())
        .bcc_recipients(bcc_recipients)
        .cc_recipients(cc_recipients)
        .build();

    info!("Email settings initialized: {:?}", email_settings);

    Ok(email_settings)
}

fn config_render() {
    set_global_render_config(
        RenderConfig::default_colored().with_canceled_prompt_indicator(
            inquire::ui::Styled::new("Skipped")
                .with_style_sheet(StyleSheet::new().with_fg(inquire::ui::Color::LightBlue)),
        ),
    );
}

pub fn ask_for_data(default: Data, data_selector: Option<DataSelector>) -> Result<Data> {
    config_render();

    fn select_or_default<T, F>(
        selector: Option<DataSelector>,
        target: DataSelector,
        default: &T,
        builder: F,
    ) -> Result<T>
    where
        F: FnOnce(&T) -> Result<T>,
        T: Clone,
    {
        if selector
            .map(|s| s.includes(target))
            .unwrap_or(selector.is_none())
        {
            builder(default)
        } else {
            Ok(default.clone())
        }
    }

    let vendor = select_or_default(data_selector, DataSelector::Vendor, default.vendor(), |d| {
        build_company("Your company", d)
    })?;

    let client = select_or_default(data_selector, DataSelector::Client, default.client(), |d| {
        build_company("Your client", d)
    })?;

    let invoice_info = select_or_default(
        data_selector,
        DataSelector::Information,
        default.information(),
        build_invoice_info,
    )?;

    let payment_info = select_or_default(
        data_selector,
        DataSelector::PaymentInfo,
        default.payment_info(),
        build_payment_info,
    )?;

    let service_fees = select_or_default(
        data_selector,
        DataSelector::ServiceFees,
        default.service_fees(),
        build_service_fees,
    )?;

    let data = Data::builder()
        .client(client)
        .vendor(vendor)
        .payment_info(payment_info)
        .service_fees(service_fees)
        .information(invoice_info)
        .expensed_months(default.expensed_months().clone())
        .build();

    Ok(data)
}
