#let provide() = {
  (
  client_info: (
    to_company: "To:",
    vat_number: "VAT:",
  ),
  invoice_info: (
    client_contact: "For the attention of:",
    due_date: "Due date:",
    invoice_date: "Invoice date:",
    invoice_identifier: "Invoice no:",
    purchase_order: "Purchase order:",
    terms: "Terms:",
    vendor_contact: "Our reference:",
  ),
  line_items: (
    description: "Item",
    grand_total: "Grand Total:",
    quantity: "Quantity",
    total_cost: "Total cost",
    unit_price: "Unit price",
    when: "When",
  ),
  month_names: (
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ),
  vendor_info: (
    address: "Address",
    bank: "Bank",
    bic: "BIC",
    iban: "IBAN",
    organisation_number: "Org. No.",
    vat_number: "VAT No.",
  ),
)
}
