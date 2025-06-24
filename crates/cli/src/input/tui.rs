use crate::prelude::*;
use inquire::{CustomType, DateSelect, Text, error::InquireResult};

const HOW_TO_SKIP_INSTRUCTION: &str = "Skip with ESC";

fn build_postal_address(owner: impl AsRef<str>) -> InquireResult<PostalAddress> {
    let sample = PostalAddress::sample();
    let text = |part: &str| format!("{}'s {part} [postal address]?", owner.as_ref());

    let zip = Text::new(&text("ZIP code"))
        .with_default(sample.zip())
        .prompt()?;

    let city = Text::new(&text("City"))
        .with_default(sample.city())
        .prompt()?;

    let country = Text::new(&text("Country"))
        .with_default(sample.country())
        .prompt()?;

    let street_line1 = Text::new(&text("Street Line 1"))
        .with_default(sample.street_address().line_1())
        .prompt()?;
    let street_line2 = Text::new(&text("Street Line 2"))
        .with_default(sample.street_address().line_2())
        .with_help_message(&format_help_skippable(
            "e.g. C/o or Apartment 12".to_owned(),
        ))
        .prompt_skippable()?
        .unwrap_or("".to_owned());

    let street_address = StreetAddress::builder()
        .line_1(street_line1)
        .line_2(street_line2)
        .build();

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
            .with_help_message(&format_help_skippable(
                if owner.to_lowercase().contains("client") {
                    "Your reference".to_owned()
                } else {
                    "Our reference".to_owned()
                },
            ))
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

fn format_help_skippable(help: impl Into<Option<String>>) -> String {
    help.into().map_or_else(
        || HOW_TO_SKIP_INSTRUCTION.to_owned(),
        |h| format!("{HOW_TO_SKIP_INSTRUCTION}: {h}"),
    )
}

fn build_year_month_inner(help: impl Into<Option<String>>) -> InquireResult<Option<YearAndMonth>> {
    let current = YearAndMonth::current();

    let help_message = format_help_skippable(help);

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
        let invoice_number_offset = CustomType::<InvoiceNumber>::new(
            "What is the last invoice number you issued? We call this the 'offset'",
        )
        .with_help_message(&format_help_skippable(
            "Used with the date of that invoice to calculate future invoice numbers.".to_owned(),
        ))
        .with_default(InvoiceNumber::default())
        .prompt_skippable()?
        .unwrap_or_default();

        let invoice_number_offset_month = build_year_month_inner(
            "When was that invoice issued? (Used to calculate future invoice numbers)".to_owned(),
        )?
        // if we use `0` as offset and set month to last month, then the next invoice number will be `1` for this month, which is correct.
        .unwrap_or(YearAndMonth::last());

        let offset = TimestampedInvoiceNumber::builder()
            .offset(invoice_number_offset)
            .month(invoice_number_offset_month)
            .build();

        let purchase_order = CustomType::<PurchaseOrder>::new("Purchase order number (optional)")
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
            .with_help_message(&format_help_skippable(
                "This is used to emphasize certain parts of the invoice, e.g. '#e6007a'".to_owned(),
            ))
            .prompt_skippable()?;

        let info = ProtoInvoiceInfo::builder()
            .offset(offset)
            .purchase_order(purchase_order)
            .footer_text(footer_text)
            .emphasize_color_hex(emphasize_color_hex)
            .months_off_record(MonthsOffRecord::default())
            .build();

        Ok(info)
    }
    inner().map_err(|e| Error::InvalidInvoiceInfo {
        reason: format!("{:?}", e),
    })
}

fn build_payment_info() -> Result<PaymentInformation> {
    fn inner() -> InquireResult<PaymentInformation> {
        let sample = PaymentInformation::sample();
        let text = |part: &str| format!("Payment {part}?");
        let bank_name = Text::new(&text("Bank Name"))
            .with_default(sample.bank_name())
            .prompt()?;
        let iban = Text::new(&text("IBAN"))
            .with_default(sample.iban())
            .prompt()?;

        let bic = Text::new(&text("BIC"))
            .with_default(sample.bic())
            .prompt()?;

        let currency = CustomType::<Currency>::new("Currency?")
            .with_help_message("The currency you want to use for the invoice, e.g. 'EUR'")
            .with_default(*sample.currency())
            .prompt()?;

        let payment_terms = CustomType::<PaymentTerms>::new("Payment terms?")
            .with_help_message("The payment terms for this invoice, e.g. 'Net 30'")
            .with_default(PaymentTerms::net30())
            .prompt()?;

        let payment_info = sample
            .with_bank_name(bank_name)
            .with_iban(iban)
            .with_bic(bic)
            .with_currency(currency)
            .with_terms(payment_terms);

        Ok(payment_info)
    }
    inner().map_err(|e| Error::InvalidPaymentInfo {
        reason: format!("{:?}", e),
    })
}

fn build_service_fees() -> Result<ServiceFees> {
    fn inner() -> InquireResult<ServiceFees> {
        let sample = ServiceFees::sample();
        let text = |part: &str| format!("Service {part}?");
        let name = Text::new(&text("Name"))
            .with_default(sample.name())
            .prompt()?;

        let unit_price = CustomType::<UnitPrice>::new("Unit price?")
            .with_help_message("The price per day, e.g. '1000'")
            .with_default(*sample.unit_price())
            .prompt()?;

        let service_fees = sample.with_name(name).with_unit_price(unit_price);

        Ok(service_fees)
    }
    inner().map_err(|e| Error::InvalidServiceFees {
        reason: format!("{:?}", e),
    })
}

pub fn ask_for_data() -> Result<Data> {
    let vendor = build_company("Your company")?;
    let client = build_company("Your client")?;
    let invoice_info = build_invoice_info()?;
    let payment_info = build_payment_info()?;
    let service_fees = build_service_fees()?;

    let data = Data::builder()
        .client(client)
        .vendor(vendor)
        .payment_info(payment_info)
        .service_fees(service_fees)
        .information(invoice_info)
        .expensed_months(ExpensedMonths::default())
        .build();

    Ok(data)
}
