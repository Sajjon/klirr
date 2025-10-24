#![cfg_attr(not(test), forbid(unsafe_code))]

mod dispatch_command;
mod init_logging;
mod input;
mod run;

pub mod prelude {
    pub(crate) use clap::{Parser, Subcommand};
    pub(crate) use derive_more::FromStr;
    pub(crate) use klirr_core_invoice::prelude::*;

    pub(crate) use crate::dispatch_command::*;
    pub(crate) use crate::init_logging::*;
    pub(crate) use crate::input::*;
    pub(crate) use crate::run::*;
}

fn main() {
    use clap::Parser;
    use prelude::*;
    init_logging();
    let input = CliArgs::parse();
    run(input)
}
