use crate::prelude::*;

use clap::{Args, Parser};
use derive_more::{Debug, Unwrap};

/// The root argument for the CLI, which contains the subcommands for
/// generating invoices and managing data.
#[derive(Debug, Parser)]
#[command(name = BINARY_NAME, about = "Generate and manage invoices")]
pub struct CliArgs {
    /// The command to run, either for generating an invoice or for data management.
    #[command(subcommand)]
    pub command: Command,
}

/// The commands available in the CLI, which include generating invoices
/// and performing data management tasks.
#[derive(Debug, Subcommand, Unwrap)]
pub enum Command {
    /// The CLI arguments for generating an invoice PDF.
    Invoice(InvoiceInput),

    /// CLI arguments for admin tasks related to data.
    Data(DataAdminInput),
}

/// The CLI arguments for data management, such as initializing the data directory,
/// validating the data, or recording expenses or month off.
#[derive(Debug, Args, Getters, PartialEq)]
pub struct DataAdminInput {
    /// The command to run for data management, such as initializing the data directory,
    /// validating the data, or recording expenses or month off.
    #[command(subcommand)]
    #[getset(get = "pub")]
    command: DataAdminInputCommands,
}

/// The commands available for data management, such as initializing the data directory,
/// validating the data, or recording expenses or month off.
#[derive(Debug, Subcommand, Unwrap, PartialEq)]
pub enum DataAdminInputCommands {
    /// Initializes the data in the data directory, creating it if it does not exist.
    /// Such as information about you as a vendor and your client, payment information
    /// pricing etc
    Init,
    /// Validates the data in the data directory, checking if it is correctly formatted
    /// and if all required fields are present.
    Validate,
    /// Records a month off for the specified month, which is used to calculate the invoice.
    MonthOff(MonthOffInput),
    /// Records expenses for the specified month, used to create expenses invoices
    /// and affects invoice number calculation.
    Expenses(ExpensesInput),
}

/// Record a new month off for the specified month.
#[derive(Debug, Args, Getters, PartialEq)]
pub struct MonthOffInput {
    /// The month to be added if not already present in the data directory.
    #[arg(
        long,
        short = 'm',
        default_value = None,
        help = "The month and year for which you wanna record a month off, e.g. `2025-05`."
    )]
    #[getset(get = "pub")]
    month: YearAndMonth,
}

/// Record expenses for the specified month, which will be used to create expenses invoices
/// and affects invoice number calculation.
#[derive(Debug, Args, Getters, PartialEq)]
pub struct ExpensesInput {
    /// The month for which the expenses are recorded.
    #[arg(
        long,
        short = 'm',
        default_value = None,
        help = "The month and year for which you wanna record expenses, e.g. `2025-05`. Note that we might expense for month of May even thought we had an expense in beginning of June, so this is not a strict month, but rather a month in which we want to record the expenses."
    )]
    #[getset(get = "pub")]
    month: YearAndMonth,

    /// The expenses to record for the month, which are specified as a list of items.
    /// Please note that the transaction date might be different from the month specified,
    /// so you can record expenses for a month even if the transaction date is in the next
    /// month, e.g. you can record expenses for May even if the transaction date is in June.
    /// Format for each item is: `name,amount,currency,quantity,date`, e.g. `Coffee,2.5,EUR,3.0,2025-05-31`.
    #[arg(long, short = 'e', help = "The expenses to record for the month.")]
    #[getset(get = "pub")]
    expenses: Vec<Item>,
}

/// The CLI arguments for generating an invoice PDF.
#[derive(Debug, Clone, TypedBuilder, Getters, Parser)]
#[command(name = "invoice")]
#[command(about = "Generate an invoice PDF", long_about = None)]
pub struct InvoiceInput {
    /// The month for which the invoice is generated.
    #[arg(long, short = 'm', default_value_t = TargetMonth::Last)]
    #[builder(setter(into), default = TargetMonth::Last)]
    #[getset(get = "pub")]
    month: TargetMonth,

    /// The language for which the invoice is generated.
    #[arg(long, short = 'l', default_value_t = Language::EN)]
    #[builder(setter(into), default = Language::EN)]
    #[getset(get = "pub")]
    language: Language,

    /// The items to be invoiced, either expenses our consulting services
    /// with an optional number of days off.
    #[command(subcommand)]
    #[builder(setter(into, strip_option), default = None)]
    #[getset(get = "pub")]
    items: Option<TargetItems>,

    /// An optional override of where to save the output PDF file.
    #[arg(long, short = 'o')]
    #[builder(setter(into, strip_option), default = None)]
    out: Option<PathBuf>,
}

impl InvoiceInput {
    /// Maps `Option<TargetItems>` to `InvoicedItems`, e.g. for `TargetItems::Ooo { days }`
    /// we map from `Option<u8>` to `Option<Day>`.
    fn _invoiced_items(&self) -> Result<InvoicedItems> {
        match self.items.clone().unwrap_or_default() {
            TargetItems::Ooo { days } => Ok(InvoicedItems::Service {
                days_off: if days == 0 {
                    None
                } else {
                    Some(Day::try_from(days)?)
                },
            }),
            TargetItems::Expenses => Ok(InvoicedItems::Expenses),
        }
    }

    /// Returns a `ValidInput` from the parsed command line arguments.
    /// This function validates the input, e.g. checks if the output path exists,
    /// and returns a `ValidInput` that can be used to generate the invoice.
    ///
    /// # Errors
    /// Returns an error if the input is invalid, e.g. if the output path does not
    /// exist or if the items are not specified correctly.
    pub fn parsed(self) -> Result<ValidInput> {
        if let Some(path) = &self.out {
            let parent = path
                .parent()
                .expect("Invalid path specified, no parent found, don't specify an empty path, a root or a prefix.");
            if !parent.exists() {
                Err(Error::SpecifiedOutputPathDoesNotExist {
                    path: path.display().to_string(),
                })?;
            }
        }
        let items = self._invoiced_items()?;
        let valid = ValidInput::builder()
            .month(self.month.year_and_month())
            .items(items)
            .language(*self.language())
            .maybe_output_path(self.out)
            .build();
        Ok(valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod data_admin_input {
        use super::*;

        #[test]
        fn test_data_admin_init() {
            let input = CliArgs::parse_from([BINARY_NAME, "data", "init"]);
            assert!(matches!(
                input.command,
                Command::Data(DataAdminInput {
                    command: DataAdminInputCommands::Init
                })
            ));
        }

        #[test]
        fn test_data_admin_validate() {
            let input = CliArgs::parse_from([BINARY_NAME, "data", "validate"]);
            assert!(matches!(
                input.command,
                Command::Data(DataAdminInput {
                    command: DataAdminInputCommands::Validate
                })
            ));
        }

        #[test]
        fn test_data_admin_expense() {
            let item_1_str = "Coffee,2.5,EUR,3.0,2025-05-31";
            let item_1 = Item::from_str(item_1_str).unwrap();
            let item_2_str = "Lunch,10.0,USD,1.0,2025-05-31";
            let item_2 = Item::from_str(item_2_str).unwrap();
            let input = CliArgs::parse_from([
                BINARY_NAME,
                "data",
                "expenses",
                "--month",
                "2025-05",
                "-e",
                item_1_str,
                "-e",
                item_2_str,
            ]);
            assert_eq!(
                *input.command.unwrap_data().command(),
                DataAdminInputCommands::Expenses(ExpensesInput {
                    month: YearAndMonth::from_str("2025-05").unwrap(),
                    expenses: vec![item_1, item_2]
                })
            );
        }
    }

    mod invoice_input {
        use super::*;

        mod tests_input {
            use super::*;
            use test_log::test;

            #[test]
            fn test_input_parsing_month() {
                let input = CliArgs::parse_from([BINARY_NAME, "invoice", "--month", "last"]);
                assert_eq!(input.command.unwrap_invoice().month, TargetMonth::Last);
            }

            #[test]
            fn test_input_parsing_language_specified() {
                let input = CliArgs::parse_from([BINARY_NAME, "invoice", "--language", "swedish"]);
                assert_eq!(input.command.unwrap_invoice().language, Language::SV);
            }

            #[test]
            fn test_input_parsing_language_default() {
                let input = CliArgs::parse_from([BINARY_NAME, "invoice"]);
                assert_eq!(input.command.unwrap_invoice().language, Language::EN);
            }

            #[test]
            fn test_input_parsing_items_specified_ooo() {
                let input = CliArgs::parse_from([BINARY_NAME, "invoice", "ooo", "3"]);
                assert_eq!(
                    input.command.unwrap_invoice().items,
                    Some(TargetItems::Ooo { days: 3 })
                );
            }

            #[test]
            fn test_input_parsing_items_specified_expenses() {
                let input = CliArgs::parse_from([BINARY_NAME, "invoice", "expenses"]);
                assert_eq!(
                    input.command.unwrap_invoice().items,
                    Some(TargetItems::Expenses)
                );
            }

            #[test]
            fn test_input_parsing_items_default() {
                let input = CliArgs::parse_from([BINARY_NAME, "invoice"]);
                assert_eq!(input.command.unwrap_invoice().items, None);
            }

            #[test]
            fn test_input_parsing_out_specified() {
                let input =
                    CliArgs::parse_from([BINARY_NAME, "invoice", "--out", "/tmp/invoice.pdf"]);
                assert_eq!(
                    input.command.unwrap_invoice().out,
                    Some(PathBuf::from("/tmp/invoice.pdf"))
                );
            }

            #[test]
            fn test_input_parsing_out_default() {
                let input = CliArgs::parse_from([BINARY_NAME, "invoice"]);
                assert_eq!(input.command.unwrap_invoice().out, None);
            }
        }

        mod tests_parsed_input {
            use super::*;
            use test_log::test;

            #[test]
            fn test_input_parsing_items_services() {
                let input = InvoiceInput::builder()
                    .items(TargetItems::Ooo { days: 25 })
                    .build();
                let input = input.parsed().unwrap();
                assert_eq!(
                    *input.items(),
                    InvoicedItems::Service {
                        days_off: Some(Day::try_from(25).unwrap())
                    }
                );
            }

            #[test]
            fn test_input_parsing_items_expenses() {
                let input = InvoiceInput::builder().items(TargetItems::Expenses).build();
                let input = input.parsed().unwrap();
                assert_eq!(*input.items(), InvoicedItems::Expenses);
            }

            #[test]
            fn test_input_parsing_out() {
                let input = InvoiceInput::builder().out("/tmp/invoice.pdf").build();
                let input = input.parsed().unwrap();
                assert_eq!(
                    *input.maybe_output_path(),
                    Some(PathBuf::from("/tmp/invoice.pdf"))
                );
            }

            #[test]
            #[should_panic]
            fn test_input_parsing_out_at_root_crashes() {
                let input = InvoiceInput::builder().out("/").build();
                let _ = input.parsed();
            }
        }
    }
}
