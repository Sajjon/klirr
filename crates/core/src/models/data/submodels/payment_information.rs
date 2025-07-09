use crate::prelude::*;

/// Bank account details for the vendor, used for international transfers.
/// This includes the IBAN, bank name, and BIC.
/// This is used to ensure that the client can pay the invoice correctly.
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Builder, Getters, WithSetters,
)]
pub struct PaymentInformation {
    /// The IBAN (International Bank Account Number) of the vendor's bank account,
    #[getset(get = "pub", set_with = "pub")]
    iban: String,

    /// The name of the vendor's bank, used for international transfers.
    #[getset(get = "pub", set_with = "pub")]
    bank_name: String,

    /// The BIC (Bank Identifier Code) of the vendor's bank, used for international
    #[getset(get = "pub", set_with = "pub")]
    bic: String,

    /// The currency of this invoice, e.g. `EUR`
    #[getset(get = "pub", set_with = "pub")]
    currency: Currency,

    /// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
    #[getset(get = "pub", set_with = "pub")]
    terms: PaymentTerms,
}

impl HasSample for PaymentInformation {
    fn sample() -> Self {
        Self::builder()
            .bank_name("Banque de Paris".into())
            .iban("FR76 3000 6000 0112 3456 7890 189".into())
            .bic("BNPAFRPP".into())
            .currency(Currency::EUR)
            .terms(PaymentTerms::sample())
            .build()
    }
}
