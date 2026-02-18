use crate::{
    CliArgs, Command, curry1, render_invoice_sample, render_invoice_sample_with_nonce,
    run_data_command, run_email_command, run_invoice_command,
};
use log::error;

use std::env::consts::OS;

/// Opens the file at `path`.
fn open_file_at(path: impl AsRef<std::path::Path>) {
    let path = path.as_ref().display().to_string();
    let result = match OS {
        "macos" => std::process::Command::new("open").arg(&path).spawn(),
        "linux" => std::process::Command::new("xdg-open").arg(&path).spawn(),
        "windows" => std::process::Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn(),
        _ => panic!("Unsupported operating system"),
    };

    if let Err(e) = result {
        eprintln!("Failed to open path: {}", e);
    }
}

/// Run CLI program with [`CliArgs`]
pub fn run(input: CliArgs) {
    match input.command {
        Command::Email(email_input) => {
            let _ = run_email_command(
                email_input.command(),
                curry1(render_invoice_sample_with_nonce, true),
            )
            .inspect_err(|e| error!("Failed to execute email command: {}", e));
        }
        Command::Sample => {
            let _ = render_invoice_sample()
                .inspect_err(|e| {
                    error!("Error creating sample invoice: {}", e);
                })
                .inspect(|outcome| {
                    open_file_at(outcome.saved_at());
                });
        }
        Command::Invoice(invoice_input) => {
            let _ = run_invoice_command(invoice_input)
                .inspect_err(|e| error!("Error creating PDF: {}", e))
                .inspect(|outcome| {
                    open_file_at(outcome.saved_at());
                });
        }
        Command::Data(data_admin_input) => {
            let _ = run_data_command(data_admin_input.command()).inspect_err(|e| {
                error!("Error running data admin command: {}", e);
            });
        }
    }
}
