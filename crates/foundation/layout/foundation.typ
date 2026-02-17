#let hline(
  length: 100%,
  thickness: 0.2pt,
  color: black,
) = {
  block[
    #line(length: length, stroke: (thickness: thickness, paint: color))
  ]
}

#let double-line(
  length: 100%,
  thickness: 0.2pt,
  color: black,
) = {
  block[
    #hline(length: length, thickness: thickness, color: color)
    #v(-11pt)
    #hline(length: length, thickness: thickness, color: color)
  ]
}
#let format_item_date(l10n, is_expenses, date) = {
  if is_expenses {
    // For expenses, format as "YYYY-MM-DD"
    str(date)
  } else {
    // For services, format as "MMM YYYY"
    let parts = str.split(date, "-")
    let year = parts.at(0)
    let month = int(parts.at(1))
    l10n.month_names.at(month - 1) + " " + year
  }
}

// Function to format numbers to two decimals
#let format_amount(amount, currency) = {
  let amt = amount * 1.0
  let integer = calc.floor(amt)
  let frac = int(calc.round((amt - integer) * 100, digits: 0))
  let frac_str = str(frac)
  if frac < 10 { frac_str = "0" + frac_str }
  let without_currency = str(integer) + "." + frac_str
  without_currency + " " + currency
}

#let display_if_non_empty(value) = {
  if value != "" {
    value
  }
}

#let footnotesize(content) = {
  set text(size: 9pt)
  content
}

#let small(content) = {
  set text(size: 10pt)
  content
}

#let normalsize(content) = {
  set text(size: 11pt)
  content
}

#let large(content) = {
  set text(size: 12pt)
  content
}

#let Large(content) = {
  set text(size: 13pt)
  content
}

#let LARGE(content) = {
  set text(size: 20pt)
  content
}

// Wraps content in a rounded box with a stroke and fill.
#let ovalbox(width, content) = {
  box(
    inset: 12pt,
    radius: 8pt,
    width: width,
    stroke: 0.2pt + black,
    fill: none,
    content,
  )
}
