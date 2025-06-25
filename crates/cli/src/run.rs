use crate::prelude::*;

pub fn run(input: CliArgs) {
    match input.command {
        Command::Invoice(invoice_input) => {
            let _ = run_invoice_command(invoice_input)
                .inspect_err(|e| error!("Error creating PDF: {}", e));
        }
        Command::Data(data_admin_input) => {
            let _ = run_data_command(data_admin_input.command()).inspect_err(|e| {
                error!("Error running data admin command: {}", e);
            });
        }
    }
}
