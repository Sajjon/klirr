use crate::dispatch_command::run_resume_command;
use crate::migration_guides::requires_manual_data_migration;
use crate::{
    CliArgs, CliResult, Command, Error, curry1, data_dir, email_settings_path,
    render_invoice_sample, render_invoice_sample_with_nonce, run_data_command, run_email_command,
    run_invoice_command,
};
use log::{error, warn};
use std::path::Path;

use std::env::consts::OS;

pub const DATA_INIT_HINT: &str =
    "💡 You seem to not have set up klirr, run `klirr data init` to get started";
pub const EMAIL_INIT_HINT: &str =
    "💡 You seem to not have set up email, run `klirr email init` to get started";

fn has_missing_setup_data(error: &Error) -> bool {
    matches!(
        error,
        Error::Core(klirr_core_invoice::Error::FileNotFound { path, .. })
            if Path::new(path).starts_with(data_dir())
    )
}

fn has_missing_email_setup_data(error: &Error) -> bool {
    matches!(
        error,
        Error::Core(klirr_core_invoice::Error::FileNotFound { path, .. })
            if Path::new(path) == email_settings_path(data_dir())
    )
}

fn log_data_setup_hint_or_error(context: &str, error: &Error) {
    if let Some(migration) = requires_manual_data_migration(error) {
        let instructions = migration.instructions();
        if !instructions.is_empty() {
            warn!("{}", instructions);
        }
    } else if has_missing_email_setup_data(error) {
        warn!("{}", EMAIL_INIT_HINT);
    } else if has_missing_setup_data(error) {
        warn!("{}", DATA_INIT_HINT);
    } else {
        error!("{context}: {error}");
    }
}

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
pub fn run(input: CliArgs) -> CliResult<()> {
    match input.command {
        Command::Resume => {
            run_resume_command()
                .inspect_err(|e| error!("Failed to execute resume command: {}", e))?;
        }
        Command::Email(email_input) => {
            run_email_command(
                email_input.command(),
                curry1(render_invoice_sample_with_nonce, true),
            )
            .inspect_err(|e| error!("Failed to execute email command: {}", e))?;
        }
        Command::Sample => {
            let outcome = render_invoice_sample().inspect_err(|e| {
                error!("Error creating sample invoice: {}", e);
            })?;
            open_file_at(outcome.saved_at());
        }
        Command::Invoice(invoice_input) => {
            let outcome = run_invoice_command(invoice_input)
                .inspect_err(|e| log_data_setup_hint_or_error("Error creating PDF", e))?;
            open_file_at(outcome.saved_at());
        }
        Command::Data(data_admin_input) => {
            run_data_command(data_admin_input.command()).inspect_err(|e| {
                log_data_setup_hint_or_error("Error running data admin command", e);
            })?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_missing_email_settings_file_as_missing_email_setup() {
        let missing_email_file = email_settings_path(data_dir());
        let err = Error::Core(klirr_core_invoice::Error::FileNotFound {
            path: missing_email_file.display().to_string(),
            underlying: "No such file or directory".to_string(),
        });

        assert!(has_missing_email_setup_data(&err));
    }

    #[test]
    fn does_not_classify_non_email_file_as_missing_email_setup() {
        let missing_data_file = data_dir().join("client.ron");
        let err = Error::Core(klirr_core_invoice::Error::FileNotFound {
            path: missing_data_file.display().to_string(),
            underlying: "No such file or directory".to_string(),
        });

        assert!(!has_missing_email_setup_data(&err));
    }

    #[test]
    fn classifies_missing_file_in_data_dir_as_missing_setup() {
        let missing_data_file = data_dir().join("client.ron");
        let err = Error::Core(klirr_core_invoice::Error::FileNotFound {
            path: missing_data_file.display().to_string(),
            underlying: "No such file or directory".to_string(),
        });

        assert!(has_missing_setup_data(&err));
    }

    #[test]
    fn does_not_classify_non_file_not_found_errors_as_missing_setup() {
        let err = Error::SpecifiedOutputPathDoesNotExist {
            path: "/tmp/nowhere".to_string(),
        };

        assert!(!has_missing_setup_data(&err));
    }

    #[test]
    fn does_not_classify_missing_file_outside_data_dir_as_missing_setup() {
        let err = Error::Core(klirr_core_invoice::Error::FileNotFound {
            path: "/tmp/client.ron".to_string(),
            underlying: "No such file or directory".to_string(),
        });

        assert!(!has_missing_setup_data(&err));
    }
}
