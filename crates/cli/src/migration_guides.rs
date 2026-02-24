use crate::{Error, data_dir};
use klirr_core_invoice::Version;
#[cfg(test)]
use strum::IntoEnumIterator;

pub const DATA_MANUAL_MIGRATION_HINT: &str =
    "💡 Your klirr data version is incompatible and must be manually migrated.";

macro_rules! migration_guides {
    ($($version:ident => $path:literal),+ $(,)?) => {
        fn migration_guide_source_path(version: Version) -> &'static str {
            match version {
                $(Version::$version => $path,)+
            }
        }

        fn migration_guide_markdown(version: Version) -> &'static str {
            match version {
                $(Version::$version => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)),)+
            }
        }

        #[cfg(test)]
        const MIGRATION_GUIDES_COUNT: usize = [$(stringify!($version)),+].len();
    };
}
migration_guides! {
    V0 => "migration/v0.md",
    V1 => "migration/v1.md",
}

#[cfg(test)]
fn empty_migration_guides() -> Vec<(Version, &'static str)> {
    Version::iter()
        .map(|version| (version, migration_guide_markdown(version).trim()))
        .filter(|(_, guide)| guide.is_empty())
        .map(|(version, _)| (version, migration_guide_source_path(version)))
        .collect()
}

#[derive(Clone, Debug)]
pub(super) enum MigrationGuide {
    OldVersion(MigrationGuideOldVersion),
}
impl std::fmt::Display for MigrationGuide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.instructions())
    }
}
impl MigrationGuide {
    pub(super) fn instructions(&self) -> String {
        match self {
            MigrationGuide::OldVersion(migration) => migration.instructions(),
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct MigrationGuideOldVersion {
    from: Version,
    to: Version,
}
impl MigrationGuideOldVersion {
    pub(super) fn instructions(&self) -> String {
        let guide_markdown = migration_guide_markdown(self.to);
        let guide_source_path = migration_guide_source_path(self.to);
        format!(
            "{hint}\n\nFrom data version: {from}\nTo data version: {to}\nRON files location: {data_dir}\nMigration guide: {guide_source_path}\n\n{guide}",
            hint = DATA_MANUAL_MIGRATION_HINT,
            from = self.from,
            to = self.to,
            data_dir = data_dir().display(),
            guide_source_path = guide_source_path,
            guide = guide_markdown
        )
    }
}

pub(super) fn requires_manual_data_migration(error: &Error) -> Option<MigrationGuide> {
    match error {
        Error::Core(klirr_core_invoice::Error::DataVersionMismatch { found, current }) => {
            Some(MigrationGuide::OldVersion(MigrationGuideOldVersion {
                from: *found,
                to: *current,
            }))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            MIGRATION_GUIDES_COUNT,
            Version::iter().count(),
            "Update migration_guides! when adding/removing Version variants"
        );
    }
}
