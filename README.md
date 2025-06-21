[![codecov](https://codecov.io/gh/Sajjon/invoice_typst/graph/badge.svg?token=HG6N5QPYPH)](https://codecov.io/gh/Sajjon/invoice_typst)

# Usage

Before you can use this software you need to install one font and modify a couple of config files with your information about your company ("the vendor"), customer ("client"), banking details ("payment"), pricing ("service fees"), invoice (invoice number offset).

## Font

```sh
brew tap homebrew/cask-fonts
brew install --cask font-computer-modern
```

## Config

Open the following files and replace the placeholder values with your info, you find the files in [`input/data/`](/input/data/). These files use [`RON` ("Rusty Object Notation")](https://github.com/ron-rs/ron) file format, a modern object notation superior to JSON/YAML/TOML.

1. [`vendor`](/input/data/vendor.ron) - required.
1. [`client`](/input/data/client.ron) - required.
1. [`invoice_info`](/input/data/invoice_info.ron) - required.
1. [`payment`](/input/data/payment.ron) - required.
1. [`service_fees`](/input/data/service_fees.ron) - required.
1. [`expenses`](/input/data/expenses.ron), optional, more info about this later.

## Run

Clone and run:

```sh
make
```

or:

```sh
make help
```

## Invoice for services

```sh
cargo run --bin invoice
```

or for current month instead of last:

```sh
cargo run --bin invoice -- --month current
```

or if you want to configure output:

```sh
cargo run --bin invoice -- --output
```

### Out of office for some days?

```sh
cargo run --bin invoice -- ooo 5
```

## Invoice for expenses

```sh
cargo run --bin invoice -- expenses "Coffee,4.25,GBP,2,2025-05-23" "Sandwich,8.75,EUR,1,2025-05-29"
```

or if you wanna open the PDF after successful build:

# Development

## Precommit

Install [`pre-commit` tool](https://pre-commit.com)

```sh
brew install pre-commit
```

## Imagemagick

Install [`imagemagick`](https://imagemagick.org)

```sh
brew install imagemagick
```
