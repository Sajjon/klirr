mod init_logging;
mod input;
mod run;

pub mod prelude {
    pub(crate) use clap::{Parser, Subcommand};
    pub(crate) use derive_more::FromStr;
    pub(crate) use invoice_typst_logic::prelude::*;

    pub(crate) use crate::input::*;
    pub(crate) use crate::run::*;
}

use prelude::*;

fn main() {
    init_logging::init_logging();
    let input = input::Input::parse();
    let _ = run(input).inspect_err(|e| error!("Error creating PDF: {}", e));
}
