// Attention! This is advanced typst code for rendering invoices.
// This typ file ONLY declares functions, it MUST be called by some other typ file.
// Typically we only want to call the `render` function from this file.
// In the beginning of this file we declare other helper functions which the
// render function uses. The input to the render function
// is a data structure and a localization structure, which are typst dictionary
// variables that we pass to the function - typically we create these typst
// dictionaries as strings from RON data which we format into valid typst.
//
// It is not meant that you modify this file directly, but rather that you
// modify the data and localization files that are used to generate the input
// to this function. Not a single string visible to the user is hardcoded
// in this file, everything is passed as data to the function.

// This is the main function that renders the invoice.
// It takes two parameters: data and l10n.
// - data: a dictionary containing invoice data
// - l10n: a dictionary containing localization strings
// The function uses these parameters to render the invoice layout, including
// the header, recipient information, invoice items, and footer.
// The function is designed to be called with the appropriate data and localization
// structures, typically generated from RON data or similar formats.
// The function does not return any value, it directly renders the invoice layout.
// It uses various helper functions defined above to format the content, such as
// formatting dates, amounts, and rendering lines and boxes.
#import "foundation.typ": *

#let render(data, l10n) = {
  let is_expenses = data.line_items.is_expenses

  // ** Invoice Data Variables **
  let emphasize_color = rgb(data.information.emphasize_color_hex)


  // Page setup: A4 paper, custom margins, and footer for contact details
  set page(margin: (top: 2cm, bottom: 11cm, left: 1.5cm, right: 1.5cm), footer: [
    // Wrap both items in a vertical block
    #block[
      #hline()
      #table(
        columns: (1fr, auto, auto),
        align: (left, left, left),
        stroke: none,
        [#strong(l10n.vendor_info.address)],
        [#strong(l10n.vendor_info.iban)],
        [#strong(l10n.vendor_info.organisation_number)],

        [#data.vendor.company_name], [#data.payment_info.iban], [#data.vendor.organisation_number],
        [#data.vendor.postal_address.street_address.line_1],
        [#strong(l10n.vendor_info.bank)],
        [#strong(l10n.vendor_info.vat_number)],

        [#data.vendor.postal_address.street_address.line_2], [#data.payment_info.bank_name], [#data.vendor.vat_number],

        [#data.vendor.postal_address.zip, #data.vendor.postal_address.city], [#strong(l10n.vendor_info.bic)], [],
        [#data.vendor.postal_address.country], [#data.payment_info.bic], [],
      )
      #hline()
      // Conditionally display footer text if it exists
      #if "footer_text" in data.information {
        v(25pt)
        align(center)[
          #Large[#strong(data.information.footer_text)]
        ]
      }
    ]
  ])
  set text(font: "CMU Serif", size: 11pt)

  grid(
    columns: (58%, 42%),
    // Two columns of equal width
    gutter: 0pt,
    // Space between blocks
    // Recipient address block
    block(fill: none, inset: 0pt, stroke: none, width: 100%, [
      #v(2mm)

      // ** Invoice Header Section **
      #LARGE[
        #data.vendor.company_name
      ]

      #text(l10n.client_info.to_company, weight: "bold")\
      #data.client.company_name (#data.client.organisation_number)\
      #data.client.postal_address.street_address.line_1\
      #display_if_non_empty(data.client.postal_address.street_address.line_2)
      #data.client.postal_address.city, #data.client.postal_address.country\
      #data.client.postal_address.zip\
      #v(7mm)
      #text(l10n.client_info.vat_number, weight: "bold")\
      #data.client.vat_number
    ]),
    block(fill: none, inset: 0pt, stroke: none, width: 100%, [
      // align the following block to the right margin
      #ovalbox(100%, [#Large(strong[#l10n.invoice_info.invoice_identifier]) #text(fill: emphasize_color)[#strong(str(
          data.information.number,
        ))]])
      // Conditionally display purchase order if it exists
      #if "purchase_order" in data.information {
        ovalbox(100%, [#strong[#l10n.invoice_info.purchase_order] #text(fill: emphasize_color)[#strong(
            data.information.purchase_order,
          )]])
      }
      #block(fill: none, [
        #ovalbox(49%, [#strong[#l10n.invoice_info.invoice_date] #data.information.invoice_date])
        #ovalbox(49%, [#strong[#l10n.invoice_info.due_date] #data.information.due_date])
      ])
      #if (
        "contact_person" in data.client and data.client.contact_person != none and data.client.contact_person != ""
      ) {
        block[
          #strong[#l10n.invoice_info.client_contact]
          #data.client.contact_person
          #v(-2mm)
        ]
      }
      #strong[#l10n.invoice_info.vendor_contact] #data.vendor.contact_person \
      #strong[#l10n.invoice_info.terms] #data.payment_info.terms
    ]),
  )

  v(1cm)

  // ** Invoice Items Table **
  double-line()
  // Calculate total in a scripting block
  let grand_total
  {
    grand_total = 0.0
    for it in data.line_items.items { grand_total = grand_total + it.total_cost }
  }
  v(-10pt)
  table(
    columns: (auto, auto, 1fr, auto, auto),
    align: (left, left, center, center, right),
    stroke: none,
    table.header(
      [#strong(l10n.line_items.description)],
      [#strong(l10n.line_items.when)],
      [#strong(l10n.line_items.unit_price)],
      [#strong(l10n.line_items.quantity)],
      [#strong(l10n.line_items.total_cost)],
    ),
    table.hline(stroke: 0.2pt),
    ..for row in data.line_items.items {
      (
        row.name,
        format_item_date(l10n, is_expenses, row.transaction_date),
        format_amount(row.unit_price, row.currency),
        str(row.quantity),
        format_amount(row.total_cost, row.currency),
        table.hline(stroke: (thickness: 0.2pt, dash: "dashed")),
      )
    },
  )
  // Grand Total Row
  align(right)[
    #set text(weight: "bold")
    #l10n.line_items.grand_total
    #set text(fill: emphasize_color)
    #format_amount(grand_total, data.payment_info.currency)
  ]
  v(-5pt)
  double-line()

  v(30pt)

  // Conditionally display the purchase order if it exists
  if "purchase_order" in data.information {
    ovalbox(100%, [
      #Large([#strong(l10n.invoice_info.purchase_order) #text(fill: emphasize_color)[#strong(
          data.information.purchase_order,
        )]])
    ])
  }
}
