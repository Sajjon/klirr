#![allow(unused)]
mod commands;
mod init_logging;
mod input;

pub mod prelude {
    pub(crate) use clap::{Parser, Subcommand};
    pub(crate) use derive_more::FromStr;
    pub(crate) use invoice_typst_logic::prelude::*;

    pub(crate) use crate::commands::*;
    pub(crate) use crate::input::*;
}

use prelude::*;

fn run(input: input::CliArgs) {
    match input.command {
        input::Commands::Invoice(invoice_input) => {
            let _ =
                run_invoice_cmd(invoice_input).inspect_err(|e| error!("Error creating PDF: {}", e));
        }
        input::Commands::Data(data_admin_input) => {
            let _ = run_data_cmd(data_admin_input).inspect_err(|e| {
                error!("Error running data admin command: {}", e);
            });
        }
    }
}

fn main() {
    init_logging::init_logging();
    let input = input::CliArgs::parse();
    run(input)
}
