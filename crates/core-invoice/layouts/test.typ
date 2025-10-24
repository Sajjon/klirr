#let render_invoice(data, l18n) = {
  block(fill: none, inset: 0pt, stroke: none, width: 100%, [
    #set text(font: "CMU Serif", size: 12pt)
    The following line should say 'Invoice number:' (if language is set to English)\
    #l18n.invoice_info.invoice_identifier

    #let emphasize_color = rgb(data.information.emphasize_color_hex)
    #text(fill: emphasize_color)[#strong(str(
        data.information.number,
      ))]
  ])
}
