use crate::prelude::*;

use clap::{Args, Parser};

#[derive(Debug, Parser)]
#[command(name = "inrost", about = "Generate and manage invoices")]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// The CLI arguments for generating an invoice PDF.
    Invoice(InvoiceInput),

    /// CLI arguments for admin tasks related to data.
    Data(DataAdminInput),
}

#[derive(Debug, Args, Getters)]
pub struct DataAdminInput {
    #[command(subcommand)]
    #[getset(get = "pub")]
    command: DataAdminInputCommands,
}

#[derive(Debug, Subcommand)]
pub enum DataAdminInputCommands {
    Init(DataInitInput),
    Validate(DataValidateInput),
    MonthOff(MonthOffInput),
    Expenses(ExpensesInput),
}

#[derive(Debug, Args, Getters)]
pub struct MonthOffInput {
    #[arg(
        long,
        short = 'm',
        default_value = None,
        help = "The month and year for which you wanna record a month off, e.g. `2025-05`."
    )]
    #[getset(get = "pub")]
    month: YearAndMonth,
}

#[derive(Debug, Args, Getters)]
pub struct ExpensesInput {
    #[arg(
        long,
        short = 'm',
        default_value = None,
        help = "The month and year for which you wanna record expenses, e.g. `2025-05`. Note that we might expense for month of May even thought we had an expense in beginning of June, so this is not a strict month, but rather a month in which we want to record the expenses."
    )]
    #[getset(get = "pub")]
    month: YearAndMonth,

    #[arg(long, short = 'e', help = "The expenses to record for the month.")]
    #[getset(get = "pub")]
    expenses: Vec<Item>,
}

pub trait DataAdminInputExt {
    /// Returns the path to the data directory to initialize or validate.
    fn path(&self) -> Option<PathBuf>;
    fn data_dir(&self) -> PathBuf {
        self.path()
            .unwrap_or(invoice_typst_logic::prelude::data_dir())
    }
}
impl DataAdminInputExt for DataInitInput {
    fn path(&self) -> Option<PathBuf> {
        self.data_dir.clone()
    }
}
impl DataAdminInputExt for DataValidateInput {
    fn path(&self) -> Option<PathBuf> {
        self.data_dir.clone()
    }
}

#[derive(Debug, Args)]
pub struct DataInitInput {
    /// The path to the data directory to initialize.
    #[arg(
        long,
        short = 'p',
        default_value = None,
        help = "If not specified, defaults `$HOME/Library/Application Support/inrost/input/data` on macOS"
    )]
    data_dir: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DataValidateInput {
    /// The path to the data directory to initialize.
    #[arg(
        long,
        short = 'p',
        default_value = None,
        help = "If not specified, defaults `$HOME/Library/Application Support/inrost/input/data` on macOS"
    )]
    data_dir: Option<PathBuf>,
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

// #[cfg(test)]
// mod tests_input {
//     use super::*;
//     use test_log::test;

//     #[test]
//     fn test_input_parsing_month() {
//         let input = Input::parse_from(["invoice", "--month", "last"]);
//         assert_eq!(input.month, TargetMonth::Last);
//     }

//     #[test]
//     fn test_input_parsing_language_specified() {
//         let input = Input::parse_from(["invoice", "--language", "swedish"]);
//         assert_eq!(input.language, Language::SV);
//     }

//     #[test]
//     fn test_input_parsing_language_default() {
//         let input = Input::parse_from(["invoice"]);
//         assert_eq!(input.language, Language::EN);
//     }

//     #[test]
//     fn test_input_parsing_items_specified_ooo() {
//         let input = Input::parse_from(["invoice", "ooo", "3"]);
//         assert_eq!(input.items, Some(TargetItems::Ooo { days: 3 }));
//     }

//     #[test]
//     fn test_input_parsing_items_specified_expenses() {
//         let input = Input::parse_from(["invoice", "expenses"]);
//         assert_eq!(input.items, Some(TargetItems::Expenses));
//     }

//     #[test]
//     fn test_input_parsing_items_default() {
//         let input = Input::parse_from(["invoice"]);
//         assert_eq!(input.items, None);
//     }

//     #[test]
//     fn test_input_parsing_out_specified() {
//         let input = Input::parse_from(["invoice", "--out", "/tmp/invoice.pdf"]);
//         assert_eq!(input.out, Some(PathBuf::from("/tmp/invoice.pdf")));
//     }

//     #[test]
//     fn test_input_parsing_out_default() {
//         let input = Input::parse_from(["invoice"]);
//         assert_eq!(input.out, None);
//     }
// }

// #[cfg(test)]
// mod tests_parsed_input {
//     use super::*;
//     use test_log::test;

//     #[test]
//     fn test_input_parsing_items_services() {
//         let input = Input::builder()
//             .items(TargetItems::Ooo { days: 25 })
//             .build();
//         let input = input.parsed().unwrap();
//         assert_eq!(
//             *input.items(),
//             InvoicedItems::Service {
//                 days_off: Some(Day::try_from(25).unwrap())
//             }
//         );
//     }

//     #[test]
//     fn test_input_parsing_items_expenses() {
//         let input = Input::builder().items(TargetItems::Expenses).build();
//         let input = input.parsed().unwrap();
//         assert_eq!(*input.items(), InvoicedItems::Expenses);
//     }

//     #[test]
//     fn test_input_parsing_out() {
//         let input = Input::builder().out("/tmp/invoice.pdf").build();
//         let input = input.parsed().unwrap();
//         assert_eq!(
//             *input.maybe_output_path(),
//             Some(PathBuf::from("/tmp/invoice.pdf"))
//         );
//     }

//     #[test]
//     #[should_panic]
//     fn test_input_parsing_out_at_root_crashes() {
//         let input = Input::builder().out("/").build();
//         let _ = input.parsed();
//     }
// }
