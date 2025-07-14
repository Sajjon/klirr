use clap::{Args, ValueEnum};
use derive_more::Unwrap;

use crate::prelude::*;

#[derive(Debug, Args, Getters, PartialEq)]
pub struct EditDataInput {
    #[arg(value_enum)]
    #[getset(get = "pub")]
    selector: EditDataInputSelector,
}

#[derive(Clone, Copy, Debug, Subcommand, Unwrap, PartialEq, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub enum EditDataInputSelector {
    All,
    Vendor,
    Client,
    Information,
    PaymentInfo,
    ServiceFees,
}

impl From<EditDataInputSelector> for DataSelector {
    fn from(selector: EditDataInputSelector) -> Self {
        match selector {
            EditDataInputSelector::All => DataSelector::All,
            EditDataInputSelector::Vendor => DataSelector::Vendor,
            EditDataInputSelector::Client => DataSelector::Client,
            EditDataInputSelector::Information => DataSelector::Information,
            EditDataInputSelector::PaymentInfo => DataSelector::PaymentInfo,
            EditDataInputSelector::ServiceFees => DataSelector::ServiceFees,
        }
    }
}
