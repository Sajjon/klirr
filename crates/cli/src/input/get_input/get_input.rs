use crate::prelude::*;

use clap::Parser;
use derive_more::{Debug, Unwrap};
use klirr_core_invoice::Layout as InvoiceLayout;

/// The root argument for the CLI, which contains the subcommands for
/// generating invoices and managing data.
#[derive(Debug, Parser)]
#[command(name = BINARY_NAME, about = "Generate invoices for services and expenses, with support for emailing them.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct CliArgs {
    /// The command to run, either for generating an invoice or for data management.
    #[command(subcommand)]
    pub command: Command,
}

/// The commands available in the CLI, which include generating invoices
/// and performing data management tasks.
#[derive(Debug, Subcommand, Unwrap)]
pub enum Command {
    /// Generate a sample invoice
    Sample,

    /// CLI arguments for email command, see [`EmailInput`].
    Email(EmailInput),

    /// CLI arguments for generating an invoice PDF, see [`InvoiceInput`].
    Invoice(InvoiceInput),

    /// CLI arguments for admin tasks related to data.
    Data(DataAdminInput),
}

/// The CLI arguments for generating an invoice PDF.
#[derive(Debug, Clone, Builder, Getters, Parser)]
#[command(name = "invoice")]
#[command(about = "Generate an invoice PDF", long_about = None)]
pub struct InvoiceInput {
    /// The period for which the invoice is generated.
    #[arg(long, short = 'p', default_value_t)]
    #[builder(default)]
    #[getset(get = "pub")]
    period: TargetPeriod,

    /// The language for which the invoice is generated.
    #[arg(long, short = 'l', default_value_t)]
    #[builder(default)]
    #[getset(get = "pub")]
    language: Language,

    /// The layout of the invoice to use
    #[arg(long, short = 't', default_value_t)]
    #[builder(default)]
    #[getset(get = "pub")]
    layout: InvoiceLayout,

    /// The items to be invoiced, either expenses our consulting services
    /// with an optional number of days off.
    #[command(subcommand)]
    #[getset(get = "pub")]
    items: Option<TargetItems>,

    /// An optional override of where to save the output PDF file.
    #[arg(long, short = 'o')]
    out: Option<PathBuf>,

    /// Whether to send the invoice via email after generating it - if
    /// the email settings are configured.
    #[arg(long, short = 'e')]
    #[builder(default = false)]
    email: bool,
}

impl InvoiceInput {
    /// Maps `Option<TargetItems>` to `InvoicedItems`.
    fn _invoiced_items(&self) -> Result<InvoicedItems> {
        match self.items.clone().unwrap_or_default() {
            TargetItems::ServicesOff(time_off) => {
                let time_off = TimeOff::try_from(time_off)?;
                Ok(InvoicedItems::Service {
                    time_off: Some(time_off),
                })
            }
            TargetItems::Services => Ok(InvoicedItems::Service { time_off: None }),
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
        let email_config = if self.email {
            validate_email_data().map(Some)
        } else {
            Ok(None)
        }?;
        let items = self._invoiced_items()?;
        let period = self.period.period();
        let valid = ValidInput::builder()
            .period(period)
            .layout(*self.layout())
            .items(items)
            .language(*self.language())
            .maybe_maybe_output_path(self.out)
            .maybe_email(email_config)
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
            assert!(matches!(input.command, Command::Data(_)));
        }

        #[test]
        fn test_data_admin_validate() {
            let input = CliArgs::parse_from([BINARY_NAME, "data", "validate"]);
            assert!(matches!(input.command, Command::Data(_)));
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
                "--period",
                "2025-05",
                "-e",
                item_1_str,
                "-e",
                item_2_str,
            ]);
            assert_eq!(
                *input.command.unwrap_data().command(),
                DataAdminInputCommand::Expenses(
                    ExpensesInput::builder()
                        .period(YearAndMonth::from_str("2025-05").unwrap().into())
                        .expenses(vec![item_1, item_2])
                        .build()
                )
            );
        }
    }

    mod invoice_input {
        use super::*;

        mod tests_input {
            use super::*;
            use test_log::test;

            #[test]
            fn test_input_parsing_period() {
                let input = CliArgs::parse_from([BINARY_NAME, "invoice", "--period", "last"]);
                assert_eq!(input.command.unwrap_invoice().period, TargetPeriod::Last);
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
            fn test_input_parsing_items_specified_services_free() {
                let input = CliArgs::parse_from([
                    BINARY_NAME,
                    "invoice",
                    "services-off",
                    "--quantity",
                    "3",
                    "--unit",
                    "days",
                ]);
                assert_eq!(
                    input.command.unwrap_invoice().items,
                    Some(TargetItems::ServicesOff(
                        TimeOffInput::builder()
                            .quantity(3.0)
                            .unit(TimeUnitInput::Days)
                            .build()
                    ))
                );
            }

            #[test]
            fn test_input_parsing_items_specified_services_not_off() {
                let input = CliArgs::parse_from([BINARY_NAME, "invoice", "services"]);
                assert_eq!(
                    input.command.unwrap_invoice().items,
                    Some(TargetItems::Services)
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
                    .items(TargetItems::ServicesOff(
                        TimeOffInput::builder()
                            .quantity(25.0)
                            .unit(TimeUnitInput::Days)
                            .build(),
                    ))
                    .build();
                let input = input.parsed().unwrap();
                let expected_decimal = Decimal::try_from(25.0).unwrap();
                let expected_quantity = Quantity::from(expected_decimal);
                assert_eq!(
                    *input.items(),
                    InvoicedItems::Service {
                        time_off: Some(TimeOff::Days(expected_quantity))
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
                let input = InvoiceInput::builder()
                    .out(PathBuf::from("/tmp/invoice.pdf"))
                    .build();
                let input = input.parsed().unwrap();
                assert_eq!(
                    *input.maybe_output_path(),
                    Some(PathBuf::from("/tmp/invoice.pdf"))
                );
            }

            #[test]
            #[should_panic]
            fn test_input_parsing_out_at_root_crashes() {
                let input = InvoiceInput::builder().out(PathBuf::from("/")).build();
                let _ = input.parsed();
            }
        }
    }

    #[test]
    fn test_data_selector_from_edit_data_input_selector() {
        let selector = EditDataInputSelector::Vendor;
        let data_selector: DataSelector = selector.into();
        assert_eq!(data_selector, DataSelector::Vendor);

        let selector = EditDataInputSelector::All;
        let data_selector: DataSelector = selector.into();
        assert_eq!(data_selector, DataSelector::All);

        let selector = EditDataInputSelector::Information;
        let data_selector: DataSelector = selector.into();
        assert_eq!(data_selector, DataSelector::Information);

        let selector = EditDataInputSelector::PaymentInfo;
        let data_selector: DataSelector = selector.into();
        assert_eq!(data_selector, DataSelector::PaymentInfo);

        let selector = EditDataInputSelector::ServiceFees;
        let data_selector: DataSelector = selector.into();
        assert_eq!(data_selector, DataSelector::ServiceFees);

        let selector = EditDataInputSelector::Client;
        let data_selector: DataSelector = selector.into();
        assert_eq!(data_selector, DataSelector::Client);
    }

    #[test]
    fn test_email_settings_selector_from_edit_email_input_selector() {
        let selector = EditEmailInputSelector::All;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(email_settings_selector, EmailSettingsSelector::All);

        let selector = EditEmailInputSelector::AppPassword;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(email_settings_selector, EmailSettingsSelector::AppPassword);

        let selector = EditEmailInputSelector::EncryptionPassword;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(
            email_settings_selector,
            EmailSettingsSelector::EncryptionPassword
        );

        let selector = EditEmailInputSelector::Template;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(email_settings_selector, EmailSettingsSelector::Template);

        let selector = EditEmailInputSelector::Smtp;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(email_settings_selector, EmailSettingsSelector::SmtpServer);

        let selector = EditEmailInputSelector::ReplyTo;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(email_settings_selector, EmailSettingsSelector::ReplyTo);

        let selector = EditEmailInputSelector::Sender;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(email_settings_selector, EmailSettingsSelector::Sender);

        let selector = EditEmailInputSelector::Recipients;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(email_settings_selector, EmailSettingsSelector::Recipients);

        let selector = EditEmailInputSelector::Cc;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(email_settings_selector, EmailSettingsSelector::CcRecipients);

        let selector = EditEmailInputSelector::Bcc;
        let email_settings_selector: EmailSettingsSelector = selector.into();
        assert_eq!(
            email_settings_selector,
            EmailSettingsSelector::BccRecipients
        );
    }
}
