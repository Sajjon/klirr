#let provide() = {
  (
  client: (
    company_name: "Holmes Ltd",
    contact_person: "Sherlock Holmes",
    organisation_number: "9876543-2101",
    postal_address: (
      city: "London",
      country: "England",
      street_address: (
        line_1: "221B Baker Street",
        line_2: "",
      ),
      zip: "NW1 6XE",
    ),
    vat_number: "GB987654321",
  ),
  information: (
    due_date: "2025-06-30",
    emphasize_color_hex: "#e6007a",
    footer_text: "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation.",
    invoice_date: "2025-05-31",
    number: 333,
    purchase_order: "PO-12345",
  ),
  line_items: (
    is_expenses: false,
    items: (
      (
        currency: "EUR",
        name: "Agreed Consulting Fees",
        quantity: 22.0,
        total_cost: 7700.0,
        transaction_date: "2025-05-31",
        unit_price: 350.0,
      ),
    ),
  ),
  output_path: (
    name: "2025-05-31_Bra_Detektiv_AB_invoice_333.pdf",
  ),
  payment_info: (
    bank_name: "SEB",
    bic: "ESSESESS",
    currency: "EUR",
    iban: "SE21 9000 0123 9876 5432 1009",
    terms: "Net 30",
  ),
  vendor: (
    company_name: "Bra Detektiv AB",
    contact_person: "Ture Sventon",
    organisation_number: "556123-4567",
    postal_address: (
      city: "Stockholm",
      country: "Sweden",
      street_address: (
        line_1: "Storgatan 45",
        line_2: "4 tr",
      ),
      zip: "114 32",
    ),
    vat_number: "SE556123456701",
  ),
)
}
