use crate::{
    CliArgs, CliResult, Command, Error, curry1, data_dir, render_invoice_sample,
    render_invoice_sample_with_nonce, run_data_command, run_email_command, run_invoice_command,
};
use klirr_core_invoice::Version;
use log::{error, warn};
use std::path::Path;
#[cfg(test)]
use strum::IntoEnumIterator;

use std::env::consts::OS;

pub const DATA_INIT_HINT: &str =
    "💡 You seem to not have set up klirr, run `klirr data init` to get started";
pub const DATA_MANUAL_MIGRATION_HINT: &str =
    "💡 Your klirr data version is incompatible and must be manually migrated.";

const EMBEDDED_MIGRATION_GUIDES: [&str; 2] = [
    include_str!("../../../docs/migration/v0.md"),
    include_str!("../../../docs/migration/v1.md"),
];

#[cfg(test)]
fn migration_guide_source_path(version: Version) -> String {
    format!("docs/migration/v{}.md", version as u16)
}

fn migration_guide_markdown(version: Version) -> &'static str {
    match version {
        Version::V0 => EMBEDDED_MIGRATION_GUIDES[0],
        Version::V1 => EMBEDDED_MIGRATION_GUIDES[1],
    }
}

#[cfg(test)]
fn empty_migration_guides() -> Vec<(Version, String)> {
    Version::iter()
        .map(|version| (version, migration_guide_markdown(version).trim()))
        .filter(|(_, guide)| guide.is_empty())
        .map(|(version, _)| (version, migration_guide_source_path(version)))
        .collect()
}

fn has_missing_setup_data(error: &Error) -> bool {
    matches!(
        error,
        Error::Core(klirr_core_invoice::Error::FileNotFound { path, .. })
            if Path::new(path).starts_with(data_dir())
    )
}

#[derive(Clone, Debug)]
struct MigrationGuideOldVersion {
    from: Version,
    to: Version,
}
impl MigrationGuideOldVersion {
    pub fn instructions(&self) -> String {
        let guide_markdown = migration_guide_markdown(self.to);
        format!(
            "{hint}\n\nFrom data version: {from}\nTo data version: {to}\nRON files location: {data_dir}\n\n{guide}",
            hint = DATA_MANUAL_MIGRATION_HINT,
            from = self.from,
            to = self.to,
            data_dir = data_dir().display(),
            guide = guide_markdown
        )
    }
}

fn requires_manual_data_migration(error: &Error) -> Option<MigrationGuideOldVersion> {
    match error {
        Error::Core(klirr_core_invoice::Error::DataVersionMismatch { found, current }) => {
            Some(MigrationGuideOldVersion {
                from: *found,
                to: *current,
            })
        }
        _ => None,
    }
}

fn log_data_setup_hint_or_error(context: &str, error: &Error) {
    if let Some(migration) = requires_manual_data_migration(error) {
        warn!("{}", migration.instructions());
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

    #[test]
    fn classifies_data_version_mismatch_as_manual_migration() {
        let err = Error::Core(klirr_core_invoice::Error::DataVersionMismatch {
            found: klirr_core_invoice::Version::V0,
            current: klirr_core_invoice::Version::current(),
        });

        assert!(requires_manual_data_migration(&err).is_some());
    }

    #[test]
    fn migration_guide_exists_for_every_version_variant() {
        let empty_guides = empty_migration_guides();
        assert!(
            empty_guides.is_empty(),
            "Missing or empty migration guides for versions: {}",
            empty_guides
                .iter()
                .map(|(version, source_path)| format!("{version} ({source_path})"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    #[test]
    fn embedded_guide_count_matches_version_count() {
        assert_eq!(
            EMBEDDED_MIGRATION_GUIDES.len(),
            Version::iter().count(),
            "Update EMBEDDED_MIGRATION_GUIDES when adding/removing Version variants"
        );
    }
}
