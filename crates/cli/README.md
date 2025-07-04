[![codecov](https://codecov.io/gh/Sajjon/klirr/graph/badge.svg?token=HG6N5QPYPH)](https://codecov.io/gh/Sajjon/klirr)

**Tired of manual bumping invoice number, calculating number of working days and looking up exchange rates and converting expenses into your currency?**

**Do you prefer automation? Then klirr is something for you!**

# Klirr

Klirr is a **SEAMLESS** (**S**etup-Once, **E**xpense-Capable, **A**esthetic, **M**onth-Idempotent, **L**ocalized, **E**xtensible, **S**cheduling-Aware, **S**elf-Sustaining) invoice solution written in Rust + [Typst](https://github.com/typst/typst)

> [!TIP]
> Scroll down to example invoice in the bottom to see what the invoice looks like.

# Description

- **S**etup Once – **One-time configuration** via an interactive Terminal UI captures all company, client, and project info. After this initial setup, no manual editing is required, and no coding or format knowledge (Rust/Typst/[RON][ron]) is needed
- **E**xpense Handling – **Expenses (even in different currencies) are integrated automatically**. You can input expenses as simple CSV-line entries, and Klirr takes care of currency conversion using up-to-date exchange rates. This spares you the hassle of looking up rates and doing conversions yourself.
- **A**esthetic Output – **Produces polished, professional invoices**. Klirr uses Typst templates to generate a beautiful invoice PDF for your services and expenses, so the final result looks as good as a hand-crafted invoice, with consistent styling and formatting.
- **M**onthly Idempotent – **Inter-month idempotence** ensures consistent invoice numbering. No matter how many times you build an invoice in a given month, it will reuse the same invoice number. When a new month begins, Klirr automatically increments to the next number. This guarantees a stable, chronological sequence of invoices without duplicates or gaps.
- **L**ocalized – **Multi-language support** is built in. Klirr dynamically loads invoice labels in different languages via i18n, and currently supports English and Swedish (adding more languages is trivial). This means your invoices can easily be generated in the language that suits you or your client.
- **E**xtensible Templates – **Flexible invoice layout with Typst**. The invoice format is powered by Typst, and while Klirr comes with one elegant layout by default, the code is prepared to very easily support additional layouts. You can extend or customize the template system to suit different styling needs, ensuring the solution can grow with your business.
- **S**cheduling-Aware – **Automatically accounts for dates and work days**. Klirr uses your system’s calendar to determine the target month’s working days and sets the invoice date to the last day of the month, with the due date calculated based on your specified payment terms. It even allows you to mark any days you were off work, subtracting those from the billable days – all handled automatically so that your invoice reflects the correct time worked.
- **S**elf-Sustaining – **Maintenance-free operation**. Klirr requires virtually no ongoing manual upkeep. Invoice numbers update themselves each month, and all calculations (dates, days, numbering, conversions) happen for you. You don’t need to remember to bump invoice numbers or adjust routine details – Klirr sustains these processes on its own, month after month.

# Installation

## Required Software

### Rust

You need [Rust](https://www.rust-lang.org/tools/install) to use this software.

## Install `klirr`

```bash
cargo install klirr
```

# Usage

You can try klirr out with sample data before you set it up with your information if you want. Try running:

```bash
klirr sample
```

## Init

Before you can use `klirr` to generate invoices you need to provide information about your company, the client, your payment details, and other info, do it using interactive terminal ui (TUI) by running:

```bash
klirr data init
```

You will be prompted

After setup is complete, you should have the following files in `$DATA_PATH/klirr/data` (
`$DATA_PATH` depends [on OS][data_path], so full path is `$HOME/Library/Application Support/klirr/data` on macOS):

1. `vendor.ron`
1. `client.ron`
1. `invoice_info.ron`
1. `payment.ron`
1. `service_fees.ron`
1. `expenses.ron`

These files use [`RON` ("Rusty Object Notation")][ron] file format, a modern object notation superior to JSON/YAML/TOML.

## Edit Data
If you later want to edit the data you input during init you can do so with another command:
```bash
klirr data edit all
```
Will start a flow similar to `init` flow, but loading in all your existing data, hit ENTER to keep using existing data, or input a new value.

Alternatively you can edit individual files using:
```bash
klirr data edit vendor
```

```bash
klirr data edit client
```

You can see the possible values with:
```bash
klirr data edit --help
```

> [!NOTE]
> `klirr data edit` does not support editing `expenses.ron` (expensed months) since
> it is an array of values and not easily edited in a simple TUI prompt.
> You append expenses using the `klirr data expenses` command, see more info
> below.

#### Manually

You can of course manually edit the files in the data folder by opening them up in your favourite text editor.

You can at any time validate the data by running:

```bash
klirr data validate
```

## Generate Invoice

```bash
klirr invoice
```

or for current month instead of last:

```bash
klirr invoice -- --month current
```

or if you want to configure output:

```bash
klirr invoice -- --output $HOME/my/custom/path/my_custom_name_of_file.pdf
```

> [!NOTE]
> If you don't specify `output` path the invoice will be saved in
> `$HOME/invoices`.

### Out of office for some days?

If you did not work for some days, and you need to not invoice for those days, e.g. `6` days off, use:

```bash
klirr invoice ooo 6
```

### Took vacation a whole month or parental leave?

You can ensure klirr uses correct invoice number calculations if you need to skip invoicing completely some months by marking said month(s) as "months off". You do it by:

```bash
klirr data month-off --month "2025-06"
```

Which will write to `$DATA_PATH/klirr/data/invoice_info.ron`

> [!TIP]
> There is currently no support for subtracting/removing expenses using Cli, if you made a mistake
> or otherwise wanna perform some changes, manually edit the file `$DATA_PATH/klirr/data/invoice_info.ron` > $DATA_PATH depends [on OS][data_path], but
> typically `$HOME/Library/Application Support` on macOS
> using your favourite text editor.
>
> After edit you can validate the data with:
>
> `cargo run --bin klirr data validate`

This ensures that there are no gaps in invoice numbers.

## Invoice for expenses

First add the expense, then generate the invoice.

### Add expenses

```bash
klirr data expenses --month 2025-05 -e "Sandwich, 6, EUR, 1, 2025-05-31" -e "Lunch, 11, GBP, 2, 2025-05-31"
```

> [!NOTE]
> The transaction day is allowed to be a different month than the value you put in `--month`, e.g. if
> if you had an expense on last of June but wanna include that expense in the invoice made in July
> should save the expense under July.

> [!NOTE]
> `klirr data expenses` will aggregate identical expenses (disregarding `quantity`) under one entry and sum
> up the quantity. So if you run the `klirr data expenses` twice with identical input and lets say one expense
> item having quantity `2`, if you run it lets say four times, it will still show as one entry but with a
> quantity of `8`.

> [!TIP]
> There is currently no support for subtracting/removing expenses using Cli, if you made a mistake
> or otherwise wanna perform some changes, manually edit the file `$DATA_PATH/klirr/data/expenses.ron` 
> where `$DATA_PATH` depends [on OS][data_path], but
> typically `$HOME/Library/Application Support` on macOS
> using your favourite text editor.
>
> After edit you can validate the data with:
>
> `cargo run --bin klirr data validate`
>
> You cannot edit expenses using `klirr data edit` as mentioned above.

### Generate expenses invoice

Then generate the expenses invoice:

```bash
klirr invoice expenses
```

> [!NOTE]
> Exchange rates will be cached in `$DATA_PATH/klirr/data/cached_rates.ron` keyed
> under the `(Date, FromCurrency, ToCurrency)` triple, to not burden the exchanges
> API unnecessarily and to make klirr extra fast for you.

# Development

Interested in development? See [development guide](DEVELOPMENT.md)

# How it works

Interested in how it works? See [explanation guide](HOW_IT_WORKS.md)

# Etymology

Klirr is a Swedish 🇸🇪 an onomatopoeia word meaning "clink" - the sound of coins falling onto a hard surface. It is part of the Swedish idiom "klirr i kassan" (_"clink in the cash register"_) meaning "cash flow", income you get from invoicing!

# Example

This is an example of the _Aioo_ `Layout` rendered using `English`.

[![Invoice Preview](crates/cli/assets/example.jpg)](crates/cli/assets/example.jpg)

[ron]: https://github.com/ron-rs/ron
[data_path]: https://docs.rs/dirs-next/latest/dirs_next/fn.data_local_dir.html
