use crate::prelude::*;
use inquire::{
    Confirm, CustomType, DateSelect, Text,
    error::InquireResult,
    set_global_render_config,
    ui::{RenderConfig, StyleSheet},
};
use rpassword::prompt_password;
use secrecy::{ExposeSecret, SecretString};

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

trait WithOptionalRefDefault<'o, T> {
    fn with_optional_ref_default(self, default: Option<&'o T>) -> Self;
}
impl<'a, 'o: 'a, T: AsRef<str>> WithOptionalRefDefault<'o, T> for Text<'a> {
    fn with_optional_ref_default(self, default: Option<&'o T>) -> Self {
        match default {
            Some(value) => self.with_default(value.as_ref()),
            None => self,
        }
    }
}
impl<'a, 'o: 'a, T: Clone> WithOptionalRefDefault<'o, T> for CustomType<'a, T> {
    fn with_optional_ref_default(self, default: Option<&'o T>) -> Self {
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

fn build_period(
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
            let help_message = format_help_skippable(help);

            let Some(half) = CustomType::<MonthHalf>::new("Half of month?")
                .with_help_message(&help_message)
                .with_optional_default(&default.and_then(|d| {
                    d.try_unwrap_year_month_and_fortnight()
                        .ok()
                        .map(|yam| *yam.half())
                }))
                .prompt_skippable()?
            else {
                return Ok(None);
            };

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

fn build_year_month_inner(
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

fn build_invoice_info(
    default: &ProtoInvoiceInfo<PeriodAnno>,
    cadence: Cadence,
) -> Result<ProtoInvoiceInfo<PeriodAnno>> {
    fn inner(
        default: &ProtoInvoiceInfo<PeriodAnno>,
        cadence: Cadence,
    ) -> InquireResult<ProtoInvoiceInfo<PeriodAnno>> {
        let invoice_number_offset = CustomType::<InvoiceNumber>::new(
            "What is the last invoice number you issued? We call this the 'offset'",
        )
        .with_help_message(&format_help_skippable(
            "Used with the date of that invoice to calculate future invoice numbers.".to_owned(),
        ))
        .with_default(default.offset().offset().clone())
        .prompt_skippable()?
        .unwrap_or_default();

        let invoice_number_offset_period = build_period(
            "When was that invoice issued? (Used to calculate future invoice numbers)".to_owned(),
            Some(default.offset().period().clone()),
            cadence,
        )?
        // if we use `0` as offset and set month to last month, then the next invoice number will be `1` for this month, which is correct.
        .unwrap_or(YearAndMonth::last().into());

        let offset = TimestampedInvoiceNumber::<PeriodAnno>::builder()
            .offset(invoice_number_offset)
            .period(invoice_number_offset_period)
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
            .maybe_purchase_order(purchase_order)
            .maybe_footer_text(footer_text)
            .maybe_emphasize_color_hex(emphasize_color_hex)
            .record_of_periods_off(default.record_of_periods_off().clone())
            .build();

        Ok(info)
    }
    inner(default, cadence).map_err(|e| Error::InvalidInvoiceInfo {
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

        let cadence = CustomType::<Cadence>::new("How often do you invoice?")
            .with_help_message("E.g. 'Monthly' or 'BiWeekly'")
            .with_default(*default.cadence())
            .prompt()?;

        let granularity = CustomType::<Granularity>::new("Do you invoice per month, day or hour? Next question will be the rate which is per time unit you provide here")
            .with_help_message("E.g. 'Month', 'Day' or 'Hour'")
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

fn ask_for_email_address_skippable(
    role: EmailAddressRole,
    default: Option<&EmailAddress>,
) -> Result<Option<EmailAddress>> {
    CustomType::<EmailAddress>::new(&format!("{}'s email address?", role))
        .with_help_message(&format_help_skippable(format!(
            "Email address for {}",
            role
        )))
        .with_optional_ref_default(default)
        .prompt_skippable()
        .map_err(|e| Error::InvalidEmailAddress {
            role: role.to_string(),
            underlying: e.to_string(),
        })
}

fn ask_for_email_address(role: EmailAddressRole, default: &EmailAddress) -> Result<EmailAddress> {
    CustomType::<EmailAddress>::new(&format!("{}'s email address?", role))
        .with_help_message(&format!("Email address for {}", role))
        .with_default(default.clone())
        .prompt()
        .map_err(|e| Error::InvalidEmailAddress {
            role: role.to_string(),
            underlying: e.to_string(),
        })
}

fn ask_for_many_email_addresses(
    role: EmailAddressRole,
    defaults: &IndexSet<EmailAddress>,
) -> Result<IndexSet<EmailAddress>> {
    let mut emails = IndexSet::new();
    loop {
        let Some(email) = ask_for_email_address_skippable(role, defaults.get_index(emails.len()))?
        else {
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

fn ask_for_email_account(role: EmailAddressRole, default: &EmailAccount) -> Result<EmailAccount> {
    let name = Text::new(&format!("Email account {} name?", role))
        .with_help_message(&format!("Will show up as the {} name", role))
        .with_default(default.name())
        .prompt()
        .map_err(|e| Error::InvalidNameForEmail {
            role: role.to_string(),
            underlying: e.to_string(),
        })?;
    let email = ask_for_email_address(role, default.email())?;
    Ok(EmailAccount::builder().name(name).email(email).build())
}

fn ask_for_email_account_skippable(
    role: EmailAddressRole,
    default: Option<&EmailAccount>,
) -> Result<Option<EmailAccount>> {
    let name = Text::new(&format!("Email account {} name?", role))
        .with_help_message(&format_help_skippable(format!(
            "Will show up as the {} name",
            role
        )))
        .with_optional_ref_default(default.as_ref().map(|d| d.name()))
        .prompt_skippable()
        .map_err(|e| Error::InvalidNameForEmail {
            role: role.to_string(),
            underlying: e.to_string(),
        })?;
    let Some(name) = name else { return Ok(None) };
    let Some(email) = ask_for_email_address_skippable(role, default.as_ref().map(|d| d.email()))?
    else {
        return Ok(None);
    };
    Ok(Some(
        EmailAccount::builder().name(name).email(email).build(),
    ))
}

fn validate(input: SecretString, min_length: usize) -> Result<SecretString> {
    let length = input.expose_secret().len();
    if length < min_length {
        Err(Error::EmailPasswordTooShort {
            min_length,
            actual_length: length,
        })
    } else {
        Ok(input)
    }
}

fn ask_for_password_once_with_length(
    prompt: &str,
    help: &str,
    min_length: usize,
    show_min_length: bool,
) -> Result<SecretString> {
    // Password from `read_password` will be zeroize at end of this function,
    // see https://github.com/conradkleinespel/rooster/pull/50/files
    let maybe_min_length_str = if show_min_length {
        format!(", min: #{} letters.", min_length)
    } else {
        "".to_owned()
    };
    prompt_password(format!("{} ({}{})", prompt, help, maybe_min_length_str))
        .map(SecretString::from)
        .map_err(|e| Error::InvalidPasswordForEmail {
            purpose: prompt.to_string(),
            underlying: e.to_string(),
        })
        .and_then(curry2(validate, min_length))
}
const PASSWORD_MIN_LENGTH: usize = 4;
fn ask_for_password_once(prompt: &str, help: &str, show_min_length: bool) -> Result<SecretString> {
    ask_for_password_once_with_length(prompt, help, PASSWORD_MIN_LENGTH, show_min_length)
}

fn ask_for_password(with_confirmation: bool, prompt: &str, help: &str) -> Result<SecretString> {
    let first = ask_for_password_once(prompt, help, with_confirmation)?;
    if !with_confirmation {
        return Ok(first);
    }
    let second = ask_for_password_once("Confirm password", help, with_confirmation)?;
    if first.expose_secret() != second.expose_secret() {
        return Err(Error::PasswordDoesNotMatch);
    }
    // second will be zeroized on drop
    Ok(first)
}

const ENV_VAR_KLIRR_EMAIL_ENCRYPTION_PASSWORD: &str = "KLIRR_EMAIL_ENCRYPTION_PASSWORD";

/// Tries to read `KLIRR_EMAIL_ENCRYPTION_PASSWORD` env variable first, if not
/// set or not at least 4 chars fallback to TUI
fn ask_for_email_encryption_password_with_confirmation(
    with_confirmation: bool,
) -> Result<SecretString> {
    if let Ok(env_pw) = std::env::var(ENV_VAR_KLIRR_EMAIL_ENCRYPTION_PASSWORD) {
        if env_pw.len() >= PASSWORD_MIN_LENGTH {
            return Ok(SecretString::from(env_pw));
        }
    }
    ask_for_password(
        with_confirmation,
        "Encryption Password",
        "Used to encrypt the SMTP App Password",
    )
}

pub fn get_email_encryption_password() -> Result<SecretString> {
    ask_for_email_encryption_password_with_confirmation(false)
}

fn ask_for_proto_email_atom_template(part: &str, default: &TemplatePart) -> Result<TemplatePart> {
    CustomType::<TemplatePart>::new(&format!("Email template for {}", part))
        .with_help_message(&TemplatePart::tutorial())
        .with_default(default.clone())
        .prompt()
        .map_err(|e| Error::EmailAtomTemplateError {
            underlying: e.to_string(),
        })
}

fn ask_for_template(default: &Template) -> Result<Template> {
    let subject = ask_for_proto_email_atom_template("subject", default.subject_format())?;
    let body = ask_for_proto_email_atom_template("body", default.body_format())?;
    Ok(Template::builder()
        .subject_format(subject)
        .body_format(body)
        .build())
}

fn config_render() {
    set_global_render_config(
        RenderConfig::default_colored().with_canceled_prompt_indicator(
            inquire::ui::Styled::new("Skipped")
                .with_style_sheet(StyleSheet::new().with_fg(inquire::ui::Color::LightBlue)),
        ),
    );
}

fn select_or_default<'a, 'b: 'a, S, T, F>(
    selector: Option<S>,
    target: S,
    default: &'b T,
    builder: F,
) -> Result<T>
where
    S: Select,
    F: FnOnce(&'a T) -> Result<T>,
    T: Clone,
{
    if selector
        .as_ref()
        .map(|s| s.includes(target))
        .unwrap_or(selector.is_none())
    {
        builder(default)
    } else {
        Ok(default.clone())
    }
}

fn ask_for_smtp_server(default: &SmtpServer) -> Result<SmtpServer> {
    CustomType::<SmtpServer>::new("SMTP server?")
        .with_help_message("The SMTP server to use for sending emails")
        .with_default(default.clone())
        .prompt()
        .map_err(|e| Error::InvalidSmtpServer {
            underlying: e.to_string(),
        })
}

pub fn ask_for_email(
    default: EncryptedEmailSettings,
    data_selector: Option<EmailSettingsSelector>,
) -> Result<EncryptedEmailSettings> {
    config_render();

    let is_editing_but_skip_secrets = data_selector
        .as_ref()
        .map(|s| !s.requires_encryption_password())
        .unwrap_or(false);

    let (salt, app_password_encrypted) = if is_editing_but_skip_secrets {
        // not edit secrets
        (default.salt().clone(), default.smtp_app_password().clone())
    } else {
        // is init or edit secrets
        let app_password_plaintext = ask_for_password(
            true,
            "SMTP App Password",
            "Used to authenticate sender account",
        )?;
        // Generate cryptographical salt
        let salt = Salt::generate();
        let encryption_password = ask_for_email_encryption_password_with_confirmation(true)?;
        let encryption_key = EncryptedAppPassword::new_by_deriving_and_encrypting(
            app_password_plaintext,
            encryption_password,
            &salt,
        );
        (salt, encryption_key)
    };

    let smtp_server = select_or_default(
        data_selector,
        EmailSettingsSelector::SmtpServer,
        default.smtp_server(),
        ask_for_smtp_server,
    )?;

    let sender = select_or_default(
        data_selector,
        EmailSettingsSelector::Sender,
        default.sender(),
        |d| ask_for_email_account(EmailAddressRole::Sender, d),
    )?;

    let template = select_or_default(
        data_selector,
        EmailSettingsSelector::Template,
        default.template(),
        ask_for_template,
    )?;

    let reply_to = select_or_default(
        data_selector,
        EmailSettingsSelector::ReplyTo,
        default.reply_to(),
        |d| ask_for_email_account_skippable(EmailAddressRole::ReplyTo, d.as_ref()),
    )?;

    let recipients = select_or_default(
        data_selector,
        EmailSettingsSelector::Recipients,
        default.recipients(),
        |d| ask_for_many_email_addresses(EmailAddressRole::Recipient, d),
    )?;

    if recipients.is_empty() {
        return Err(Error::RecipientAddressesCannotBeEmpty);
    }

    let cc_recipients = select_or_default(
        data_selector,
        EmailSettingsSelector::CcRecipients,
        default.cc_recipients(),
        |d| ask_for_many_email_addresses(EmailAddressRole::Cc, d),
    )?;

    let bcc_recipients = select_or_default(
        data_selector,
        EmailSettingsSelector::BccRecipients,
        default.bcc_recipients(),
        |d| ask_for_many_email_addresses(EmailAddressRole::Bcc, d),
    )?;

    let email_settings = EncryptedEmailSettings::builder()
        .sender(sender)
        .smtp_server(smtp_server)
        .smtp_app_password(app_password_encrypted)
        .maybe_reply_to(reply_to)
        .recipients(recipients.clone())
        .bcc_recipients(bcc_recipients)
        .cc_recipients(cc_recipients)
        .template(template)
        .salt(salt)
        .build();

    info!("Email settings initialized: {:?}", email_settings);

    Ok(email_settings)
}

pub fn ask_for_data(
    default: Data<PeriodAnno>,
    data_selector: Option<DataSelector>,
) -> Result<Data<PeriodAnno>> {
    config_render();

    let vendor = select_or_default(data_selector, DataSelector::Vendor, default.vendor(), |d| {
        build_company("Your company", d)
    })?;

    let client = select_or_default(data_selector, DataSelector::Client, default.client(), |d| {
        build_company("Your client", d)
    })?;

    let service_fees = select_or_default(
        data_selector,
        DataSelector::ServiceFees,
        default.service_fees(),
        build_service_fees,
    )?;

    let invoice_info = select_or_default(
        data_selector,
        DataSelector::Information,
        default.information(),
        curry2(build_invoice_info, *service_fees.cadence()),
    )?;

    let payment_info = select_or_default(
        data_selector,
        DataSelector::PaymentInfo,
        default.payment_info(),
        build_payment_info,
    )?;

    let data = Data::builder()
        .client(client)
        .vendor(vendor)
        .payment_info(payment_info)
        .service_fees(service_fees)
        .information(invoice_info)
        .expensed_periods(default.expensed_periods().clone())
        .build();

    Ok(data)
}
