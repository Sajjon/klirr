# How it works

This invoice solution is written in Rust + [Typst](https://github.com/typst/typst).

# Typst

Typst is three things really:

1. Typst is a [language with its own syntax](https://typst.app/docs/reference/syntax/)
1. Typst is a [Rust SDK](https://github.com/typst/typst)
1. Typst is a [Web App Editor (requires subscription)](https://typst.app/docs/web-app/)

Klirr does uses the first two.

# High level description

When you create an invoice, high level this is what Klirr does:

1. **Once**, user perform setup by inputting info into terminal UI, using `klirr data init`
1. User calls `klirr invoice` with args, e.g. `language`, `target` month,` output` (path), `layout`, and `days off` if invoicing for services, we parse these Cli arg as `InvoiceInput`. **By default** we use **last** month as `target` month, since that's when we know which days last month we worked or not.
1. Read RON riles saved upon `klirr data init`, forming `Data`
1. Process the data and perform some calculations:
    - Calculate the invoice number using `offset` and `target` month
    - Calculate the invoice date (last day of `target` month)
    - Calculate the due date (invoice day + number of days dictated by `payment.terms`)
    - **If invoice is for services:** calculate number of days being charged for by calculating the number of working days for the `target` month and subtracts and `days off`
    - **If invoice is for expenses:** perform networks call fetching exchanges rates from source currency to target currency at the dates the expenses occurred
    - Put all this processed and calculated values in a `PreparedData`
1. Load the `L18nContent` value using `language` input
1. Load the `Layout` value using `layout` input
1. Create a `main.typ` file which imports helper Typst files created from `L18nContent` and `PreparedData`. Convert Rust values to Typst dictionaries. Call Typst function called `render_invoice` declared in `Layout`: `layout.render_invoice(data, l18n)` which outputs the final Typst content.
1. Compile the Typst content from the last step in Rust using `typst::compile` creating a `PagedDocument`
1. Convert the `PagedDocument` into `Pdf` bytes
1. Depending on `output` path from input we name it and place it at a user provided absolute path, or we name it `"<INVOICE_DATE>_<VENDOR_NAME>_<INVOICE_NUMBER>.pdf"` (or including "_expense_`if expense), and save it in`$HOME/Documents/invoices/` (will be created if does not exist).
1. If the `TMP_FILE_FOR_PATH_TO_PDF` ENV variable is set, the absolute path to the Pdf is saved to a file in at that path (useful for scripts, used by `makefile` of klirr during development).

# Detailed explanation

Upon `klirr data init` you input `Data` with information about your company, client, service feeds, invoice details such as invoice number offset (together with current date, and target month used to calculate invoice number automatically) and some content specific things like color and footer text.

The input is saved in an application data path on your machine (on macOS: `$HOME/Library/Application Support/klirr`) as [RON-files][ron], e.g. `vendor.ron` and `client.ron`

Klirr is bundled with multiple language support, you control the language used on the invoice with the `--language` flag. The localization data for each language is bundled with `klirr` as pure Rust functions (e.g. `L18n::english()`).

Furthermore, Klirr is bundled with layouts (at the time of writing only one - "aioo" - is implemented, but in the future we plan to include more and allow users to select which one with `--layout` CLI flag). These layouts are written in the Typst language. The format of the layout is that they MUST declare one Typst function `render_invoice(data, l18n)`, taking two parameters, `data` and `l18n`. Data
contains all the content of your invoice, read from the RON files in the application data path on your machine. `l18n` contains the labels and is dictated by the `--language` flag.

When Klirr generates the invoice it uses Typst to render it, and calls `render_invoice` with `data` and `l18n`. But `data` is a Rust value
of type `Data` that first needs to be reformatted to a Typst dictionary returned by a Typst function called `provide()`. We map Rust object => Typst dictionary by serializing it to JSON first and then reformatting the data a bit. We do the same thing for Rust value of type `Localization` -> `l18n` Typst dictionary.

We rely on something called ["VirtualPaths" in Typst](https://docs.rs/typst/latest/typst/syntax/struct.VirtualPath.html) to perform the Typst rendering. To Typst we provide 4 VirtualPaths, 0 real paths. 0 real paths means that all Typst files (`.typ`) Klirr uses are inlined Strings, why? Because `cargo install` [only includes compiled code, not resources such as text files](https://users.rust-lang.org/t/cargo-handling-of-resource-files/109780/2?u=sajjon), e.g. `.typ` files. So the layout files, e.g. [`aioo.typ` layout file](crates/core/layouts/aioo.typ) is compiled with the binary using the `include_str!` macro.

Below is an except of the Rust `render` function, which takes three values which all conform to a `ToTypstFn` trait which requires impl of `to_typst_fn` function, which returns a String, with Typst code which declared a Typst function.

The `main.typ` file is declared using multiline Rust String literal:

```rust
    let main = format!(
        r#"
    #import "{}": provide as provide_data
    #import "{}": provide as provide_localization
    #import "{}": render_invoice
    #render_invoice(provide_data(), provide_localization())
    "#,
        TYPST_VIRTUAL_NAME_DATA, TYPST_VIRTUAL_NAME_L18N, TYPST_VIRTUAL_NAME_LAYOUT
    );

```

Which declares a four lines Typst file, which:

1. imports `provide` Typst function from the virtual file `TYPST_VIRTUAL_NAME_DATA` (`"data.typ"`) renamed as `provide_data` (for disambiguation), and the contents of `"data.typ` comes from `data.to_typst_fn()` which is a Typst function which declared a `provide` method returning the data converted to JSON reformatted to Typst Dictionary as Rust String.
1. imports `provide` Typst function from the virtual file `TYPST_VIRTUAL_NAME_L18N` (`"l18n.typ"`) renamed as `provide_localization` (for disambiguation), and is created analously, by calling `l18n.content().to_typst_fn();`.
1. imports `render_invoice` Typst function from the virtual file `TYPST_VIRTUAL_NAME_LAYOUT` (`layout.typ""`) which is analoguosly created by calling `layout.to_typst_fn();`.
1. call `#render_invoice(provide_data(), provide_localization())` which returns the Typst returned by the `layout`.

```rust
pub const TYPST_VIRTUAL_NAME_MAIN: &str = "main.typ";
pub const TYPST_VIRTUAL_NAME_LAYOUT: &str = "layout.typ";
pub const TYPST_VIRTUAL_NAME_DATA: &str = "data.typ";
pub const TYPST_VIRTUAL_NAME_L18N: &str = "l18n.typ";

/// Renders a PDF document using Typst with the provided layout, localization, and data.
pub fn render(l18n: L18n, data: PreparedData, layout: Layout) -> Result<Pdf> {
    let l18n_typst_str = l18n.content().to_typst_fn();
    let data_typst_str = data.to_typst_fn();
    let layout_typst_str = layout.to_typst_fn();
    let main = format!(
        r#"
    #import "{}": provide as provide_data
    #import "{}": provide as provide_localization
    #import "{}": render_invoice
    #render_invoice(provide_data(), provide_localization())
    "#,
        TYPST_VIRTUAL_NAME_DATA, TYPST_VIRTUAL_NAME_L18N, TYPST_VIRTUAL_NAME_LAYOUT
    );

    let context =
        TypstContext::with_inline(main, layout_typst_str, l18n_typst_str, data_typst_str)?;

    let doc = typst::compile::<PagedDocument>(&context)?;
    // convert to PDF
    typst_pdf::pdf(doc)
}
```

Why not declare a variable `data` directly? Seems we cannot import a Typst variable from another Typst (Virtual) file it seems (at the time of writing). However, we _can_ import a function.

[ron]: (https://github.com/ron-rs/ron)
