use inquire::{CustomType, Text, error::InquireResult};

use crate::{Currency, Error, PaymentInformation, PaymentTerms, Result};

pub fn build_payment_info(default: &PaymentInformation) -> Result<PaymentInformation> {
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
