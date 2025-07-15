use clap::Args;

use crate::prelude::*;

/// Record expenses for the specified period, which will be used to create expenses invoices
/// and affects invoice number calculation.
#[derive(Debug, Args, Getters, PartialEq, Builder)]
pub struct ExpensesInput {
    /// The period for which the expenses are recorded.
    #[arg(
        long,
        short = 'p',
        default_value = None,
        help = "The period for which you wanna record expenses, e.g. `2025-05`. or `2025-06-first-half`. Note that we might expense for period of May even thought we had an expense in beginning of June, so this is not a strict period, but rather a period in which we want to record the expenses."
    )]
    #[getset(get = "pub")]
    period: PeriodAnno,

    /// The expenses to record for the period, which are specified as a list of items.
    /// Please note that the transaction date might be different from the month specified,
    /// so you can record expenses for a month even if the transaction date is in the next
    /// month, e.g. you can record expenses for May even if the transaction date is in June.
    /// Format for each item is: `name,amount,currency,quantity,date`, e.g. `Coffee,2.5,EUR,3.0,2025-05-31`.
    #[arg(long, short = 'e', help = "The expenses to record for the period.")]
    #[getset(get = "pub")]
    expenses: Vec<Item>,
}
