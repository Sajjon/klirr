use getset::WithSetters;

use crate::prelude::*;

/// Bank account details for the vendor, used for international transfers.
/// This includes the IBAN, bank name, and BIC.
/// This is used to ensure that the client can pay the invoice correctly.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters, WithSetters)]
pub struct PaymentInformation {
    /// The IBAN (International Bank Account Number) of the vendor's bank account,
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    iban: String,

    /// The name of the vendor's bank, used for international transfers.
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    bank_name: String,

    /// The BIC (Bank Identifier Code) of the vendor's bank, used for international
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    bic: String,

    /// The currency of this invoice, e.g. `EUR`
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    currency: Currency,

    /// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
    #[builder(setter(into))]
    #[getset(get = "pub", set_with = "pub")]
    terms: PaymentTerms,
}

impl HasSample for PaymentInformation {
    fn sample() -> Self {
        Self::builder()
            .bank_name("SEB")
            .iban("SE21 9000 0123 9876 5432 1009")
            .bic("ESSESESS")
            .currency(Currency::EUR)
            .terms(PaymentTerms::sample())
            .build()
    }
}
