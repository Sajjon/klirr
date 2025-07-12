[![Build Status](https://github.com/Sajjon/klirr/actions/workflows/test.yml/badge.svg)](https://github.com/Sajjon/klirr/actions)
[![codecov](https://codecov.io/gh/Sajjon/klirr/graph/badge.svg?token=HG6N5QPYPH)](https://codecov.io/gh/Sajjon/klirr)
[![Latest Version](https://img.shields.io/crates/v/klirr.svg)](https://crates.io/crates/klirr)
[![Rust Documentation](https://docs.rs/klirr-render/badge.svg)](https://docs.rs/klirr-render)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/Sajjon/klirr/main/LICENSE.txt)
[![Rust 1.85.1](https://img.shields.io/badge/rustc-1.85.1-lightgray.svg)](https://blog.rust-lang.org/2025/03/18/Rust-1.85.1/)
[![Unsafe Forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance)

**Tired of manual bumping invoice number, calculating number of working days and looking up exchange rates and converting expenses into your currency?**

**Do you prefer automation? Then klirr is something for you!**

# Klirr

Klirr is an **AMAZING** (**A**esthetic, **M**ulti-layouts/-language, **A**utomatic, **Z**ero-Maintenance, **I**nter-month Idempotent, **N**imble, **G**ratis) invoice solution written in Rust + [Typst](https://github.com/typst/typst)

> [!TIP]
> Scroll down to example invoice in the bottom to see what the invoice looks like.
>
> Or try it out with `cargo install klirr && klirr sample`

# Table Of Contents <a href="#thetoc" id="thetoc"/>

<!-- disabledMarkdownTOC autolink="false" markdown_preview="github" -->

-   [Description](#description)
-   [Installation](#installation)
    -   [Pre-built binaries](#prebuilt-binaries)
        -   [brew](#brew)
    -   [Build](#manual-install)
        -   [Install Rust](#install-rust)
        -   [Install `klirr`](#install-klirr)
-   [Usage](#usage)
    -   [Init](#init)
    -   [Edit Data](#edit-data)
        -   [Manually](#data-edit-manual)
    -   [Generate Invoice](#generate-invoice)
        -   [Out of office for some days?](#ooo)
        -   [Took vacation a whole month or parental leave?](#month-off)
    -   [Invoice for expenses](#expenses)
        -   [Add expenses](#expenses-add)
        -   [Generate expenses invoice](#expenses-generate)
    -   [Email](#email)
        -   [Init](#email-init)
        -   [Send Test email](#email-test)
        -   [Security](#email-security)
-   [Development](#development)
-   [How it works](#how-it-works)
-   [Etymology](#etymology)
-   [Example](#example)

<!-- /MarkdownTOC -->

# Description<a href="#description" id="description"/>[ ^](#thetoc)

-   **A**esthetic – **Produces polished, professional invoices**. Klirr uses Typst templates to generate a beautiful invoice PDF for your services and expenses, so the final result looks as good as a hand-crafted invoice, with consistent styling and formatting.
-   **M**ultiple layouts & languages – **Localized and dynamic with support for multiple layouts**. Klirr dynamically loads invoice labels in different languages via i18n, and currently supports English and Swedish (adding more languages is trivial). This means your invoices can easily be generated in the language that suits you or your client. The invoice format is powered by Typst, and while Klirr comes with one elegant layout by default, the code is prepared to very easily support additional layouts. You can extend or customize the template system to suit different styling needs, ensuring the solution can grow with your business.
-   **A**utomatic – **Automatically accounts for dates and work days**. Klirr uses your system’s calendar to determine the target month’s working days and sets the invoice date to the last day of the month, with the due date calculated based on your specified payment terms. It even allows you to mark any days you were off work, subtracting those from the billable days – all handled automatically so that your invoice reflects the correct time worked.
-   **Z**ero-Maintenance – **One-time configuration** via an interactive Terminal UI captures all company, client, and project info. After this initial setup, no manual editing is required, and no coding or format knowledge (Rust/Typst/[RON][ron]) is needed
-   **I**nter-month Idempotent – **Inter-month idempotence** ensures consistent invoice numbering. No matter how many times you build an invoice in a given month, it will reuse the same invoice number. When a new month begins, Klirr automatically increments to the next number. This guarantees a stable, chronological sequence of invoices without duplicates or gaps.
-   **N**imble - **Quickly and effortlessly generates invoices in no-time**
-   **G**ratis – **Free and open source forever**.

If that was not enough, klirr also supports generation of expense invoices which automatically fetched currency exchange rates and translates them into your currency at the date of transaction for all your purchases (see below).

Klirr also supports automatic emailing of the invoices (see below).

# Installation<a href="#installation" id="installation"/>[ ^](#thetoc)

You can either install `klirr` from a prebuilt binary or you can build it yourself.

## Pre-built binaries<a href="#prebuilt-binaries" id="prebuilt-binaries"/>[ ^](#thetoc)

All prebuilt binaries are cryptographically signed and include build attestations for security verification. See [SECURITY.md](SECURITY.md) for details on verifying downloads.

### brew<a href="#brew" id="brew"/>[ ^](#thetoc)

```bash
brew install Sajjon/klirr/klirr
```

### Direct Download<a href="#direct-download" id="direct-download"/>[ ^](#thetoc)

You can download signed binaries directly from [GitHub Releases](https://github.com/Sajjon/klirr/releases). All releases are marked as "Verified" by GitHub and include build attestations.

## Build<a href="#manual-install" id="manual-install"/>[ ^](#thetoc)

If you wanna build your self you can do it with Rust and cargo.

### Install Rust<a href="#install-rust" id="install-rust"/>[ ^](#thetoc)

You need [Rust](https://www.rust-lang.org/tools/install) to use this software.

### Install `klirr`<a href="#install-klirr" id="install-klirr"/>[ ^](#thetoc)

```bash
cargo install klirr
```

# Usage<a href="#usage" id="usage"/>[ ^](#thetoc)

You can try klirr out with sample data before you set it up with your information if you want. Try running:

```bash
klirr sample
```

## Init<a href="#init" id="init"/>[ ^](#thetoc)

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

## Edit Data<a href="#edit-data" id="edit-data"/>[ ^](#thetoc)

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

### Manually<a href="#data-edit-manual" id="data-edit-manual"/>[ ^](#thetoc)

You can of course manually edit the files in the data folder by opening them up in your favourite text editor.

You can at any time validate the data by running:

```bash
klirr data validate
```

## Generate Invoice<a href="#generate-invoice" id="generate-invoice"/>[ ^](#thetoc)

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

### Out of office for some days? <a href="#ooo" id="ooo"/> [ ^](#thetoc)

If you did not work for some days, and you need to not invoice for those days, e.g. `6` days off, use:

```bash
klirr invoice ooo 6
```

### Took vacation a whole month or parental leave? <a href="#month-off" id="month-off"/> [ ^](#thetoc)

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

## Invoice for expenses<a href="#expenses" id="expenses"/>[ ^](#thetoc)

First add the expense, then generate the invoice.

### Add expenses<a href="#expenses-add" id="expenses-add"/>[ ^](#thetoc)

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

### Generate expenses invoice <a href="#expenses-generate" id="expenses-generate"/> [ ^](#thetoc)

Then generate the expenses invoice:

```bash
klirr invoice expenses
```

> [!NOTE]
> Exchange rates will be cached in `$DATA_PATH/klirr/data/cached_rates.ron` keyed
> under the `(Date, FromCurrency, ToCurrency)` triple, to not burden the exchanges
> API unnecessarily and to make klirr extra fast for you.

## Email<a href="#email" id="email"/>[ ^](#thetoc)

Klirr can automatically send an email with the invoice for you after it has been generated.

This requires you to setup an _App Password_ with your email service, for information on
how to set it up for [Gmail see here](https://myaccount.google.com/apppasswords).
If you setup klirr to be able to send emails you will be prompted for this _App Password_
and you will be prompted for an encryption password which will be used to encrypt
the _App Password_. The encryption password can be anything as long as it adheres to
minimum length requirement (typically 4 chars min).

### Init<a href="#email-init" id="email-init"/>[ ^](#thetoc)

Get started with email sending of your invoices by setting up the email configuration, run:

```bash
klirr email init
```

Similarly to `klirr data init` you will now be prompted for a series of inputs required to send your email, including sender email, sender _App Password_, encryption password (see Security below for more info), recipients email address and more.

Later, when using this email sending feature you will always be prompted to input the encryption password, so that klirr can decrypt the _App Password_ to be able to send emails.

> [!TIP]
> Optionally, if you don't want to have to input the encryption password every
> time you invoice you can export it in an environment variable named
> `KLIRR_EMAIL_ENCRYPTION_PASSWORD`, we recommend you use `direnv` and source
> it from a hidden file, typically `.envrc.secret` place in a directory you
> will run `klirr` from, which is safer than exporting the password in your
> `.zshrc`.

### Send Test email<a href="#email-test" id="email-test"/>[ ^](#thetoc)

You can try sending a test email using `klirr email test` (you will be prompted for you encryption password).

### Security<a href="#email-security" id="email-security"/>[ ^](#thetoc)

> [!IMPORTANT]
> Klirr's email feature is safe to use. Klirr uses strong encryption and employes
> all IT security best practices to keep your _App Password_ safe.
> It is crucual that an attack does not get access to your _App Password_ since email
> services does not allow users to limit the scope and permission of the _App Password_, with it
> and attacker can read all your emails and send emails to anyone impersonating you!

Therefor, klirr employes these best practices to keep your _App Password_ safe: 
1. Key-Derivation: Klirr does not use your encryption password directly, first it's run through a [Hash based Key-Derivation-Function](https://en.wikipedia.org/wiki/HKDF) using a application unique `INFO` (see 'How Should You Introduce Randomness into HKDF?' section of [this blog post](https://soatok.blog/2021/11/17/understanding-hkdf/)) and cryptographically secure random generated [`SALT`](https://en.wikipedia.org/wiki/Salt\_(cryptography)), this forms a strong and unique `EncryptionKey` 
2. Advanced Encryption: Klirr uses [AES (Advanced Encryption Standard)] encryption with 256 bits strength, encrypted using the `EncryptionKey` from last step. 
3. Zeroisation: Klirr uses [Zeroisation](https://en.wikipedia.org/wiki/Zeroisation) to eagerly erase sensitive secrets from memory.

You can review how klirr employes these safety measures in the [encryption folder of the code](crates/core/src/logic/encryption).

# Development<a href="#development" id="development"/>[ ^](#thetoc)

Interested in development? See [development guide](DEVELOPMENT.md)

# How it works<a href="#how-it-works" id="how-it-works"/>[ ^](#thetoc)

Interested in how it works? See [explanation guide](HOW_IT_WORKS.md)

# Etymology<a href="#etymology" id="etymology"/>[ ^](#thetoc)

Klirr is a Swedish 🇸🇪 an onomatopoeia word meaning "clink" - the sound of coins falling onto a hard surface. It is part of the Swedish idiom "klirr i kassan" (_"clink in the cash register"_) meaning "cash flow", income you get from invoicing!

# Example<a href="#example" id="example"/>[ ^](#thetoc)

This is an example of the _Aioo_ `Layout` rendered using `English`.

[![Invoice Preview](crates/cli/assets/example.jpg)](crates/cli/assets/example.jpg)

[ron]: https://github.com/ron-rs/ron
[data_path]: https://docs.rs/dirs-next/latest/dirs_next/fn.data_local_dir.html
