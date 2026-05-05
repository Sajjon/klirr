use crate::PreparedData;
use derive_more::From;
use serde_with::DeserializeFromStr;
use serde_with::SerializeDisplay;

/// A formatting taking one argument: Invoice number, e.g. "Invoice{}".
/// At time of composing the email, the subject will be
/// formatted with the invoice number.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::FromStr,
    From,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[from(String, &str)]
pub struct TemplatePart(String);
impl TemplatePart {
    const NUMBER: &str = "<INV_NO>";
    const VENDOR: &str = "<FROM_CO>";
    const CLIENT: &str = "<TO_CO>";
    const INVOICE_DATE: &str = "<INV_DATE>";
    /// The invoice's purchase-order string, or empty when none was set.
    const PURCHASE_ORDER: &str = "<PO>";

    pub fn tutorial() -> String {
        format!(
            "Placeholders: '{}', '{}', '{}', '{}', '{}'. Example: 'Invoice {} from {}' → 'Invoice 42 from Lupin et Associés'. '{}' expands to the purchase order or an empty string when none is set. Placeholders are case-sensitive and must include '<' and '>'.",
            Self::NUMBER,
            Self::VENDOR,
            Self::CLIENT,
            Self::INVOICE_DATE,
            Self::PURCHASE_ORDER,
            Self::NUMBER,
            Self::VENDOR,
            Self::PURCHASE_ORDER
        )
    }

    pub fn materialize(&self, data: &PreparedData) -> String {
        let mut raw = self.0.clone();
        raw = raw.replace(
            Self::NUMBER,
            data.information().number().to_string().as_str(),
        );
        raw = raw.replace(Self::VENDOR, data.vendor().company_name().as_str());
        raw = raw.replace(Self::CLIENT, data.client().company_name().as_str());
        raw = raw.replace(
            Self::INVOICE_DATE,
            data.information().invoice_date().to_string().as_str(),
        );
        let po = data
            .information()
            .purchase_order()
            .as_ref()
            .map(|po| po.to_string())
            .unwrap_or_default();
        raw = raw.replace(Self::PURCHASE_ORDER, po.as_str());

        #[cfg(debug_assertions)]
        {
            let rng = "<RNG>";
            if raw.contains(rng) {
                let rnd: u64 = rand::random();
                raw = raw.replace(rng, rnd.to_string().as_str());
            }
        }
        raw
    }
}

impl Default for TemplatePart {
    fn default() -> Self {
        Self(format!("Invoice {} from {}", Self::NUMBER, Self::VENDOR))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasSample;

    #[test]
    fn test_replace() {
        let template = TemplatePart::default();
        assert_eq!(template.0, "Invoice <INV_NO> from <FROM_CO>");
        let result = template.materialize(&PreparedData::sample());
        assert_eq!(result, "Invoice 9876 from Lupin et Associés");
    }

    #[test]
    fn test_that_tutorial_contains_all_variables() {
        let tutorial = TemplatePart::tutorial();
        assert!(tutorial.contains(TemplatePart::NUMBER));
        assert!(tutorial.contains(TemplatePart::VENDOR));
        assert!(tutorial.contains(TemplatePart::CLIENT));
        assert!(tutorial.contains(TemplatePart::INVOICE_DATE));
        assert!(tutorial.contains(TemplatePart::PURCHASE_ORDER));
    }

    #[test]
    fn test_purchase_order_placeholder_is_replaced_when_set() {
        let template = TemplatePart::from("PO: <PO>");
        let result = template.materialize(&PreparedData::sample());
        // PreparedData::sample() carries PurchaseOrder::sample() => "PO-12345".
        assert_eq!(result, "PO: PO-12345");
    }

    #[test]
    fn test_purchase_order_placeholder_is_empty_when_unset() {
        use crate::{
            CompanyInformation, InvoiceInfoFull, InvoiceNumber, LineItemsFlat, OutputPath,
            PaymentInformation, PreparedData,
        };
        use klirr_foundation::Date;
        // Build a PreparedData with no purchase order set.
        let info = InvoiceInfoFull::builder()
            .number(InvoiceNumber::sample())
            .invoice_date(Date::sample())
            .due_date(Date::sample())
            .build();
        let prepared = PreparedData::builder()
            .information(info)
            .vendor(CompanyInformation::sample_vendor())
            .client(CompanyInformation::sample_client())
            .line_items(LineItemsFlat::sample())
            .payment_info(PaymentInformation::sample())
            .output_path(OutputPath::Name("invoice.pdf".into()))
            .build();
        let template = TemplatePart::from("PO: '<PO>'");
        assert_eq!(template.materialize(&prepared), "PO: ''");
    }

    #[test]
    fn test_rng() {
        let template = TemplatePart::from("<RNG>");
        let result = template.materialize(&PreparedData::sample());
        let int_parsed = result.parse::<u64>();
        assert!(int_parsed.is_ok(), "Expected a number, got: {}", result);
    }
}
