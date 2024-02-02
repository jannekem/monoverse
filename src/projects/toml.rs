use std::path::PathBuf;

use crate::settings::ProjectSettings;
use crate::version::{ToVersion, VersionContext};

pub struct TomlProject {
    base: super::BaseProjectFile,
}

impl TomlProject {
    pub fn new(settings: ProjectSettings, repo_path: PathBuf) -> Self {
        Self {
            base: super::BaseProjectFile {
                settings,
                repo_path,
            },
        }
    }
}

impl super::ProjectFile for TomlProject {
    fn base(&self) -> &super::BaseProjectFile {
        &self.base
    }

    fn update_version(
        &self,
        version_file_content: &str,
        version_context: &super::VersionContext,
    ) -> anyhow::Result<String> {
        crate::edit::toml::edit(
            version_file_content,
            &self
                .base
                .settings
                .selector
                .clone()
                .ok_or(anyhow::anyhow!("Selector is required for a TOML project",))?,
            &version_context.next_version.to_string(),
        )
    }

    fn version_context(
        &self,
        version_file_content: &str,
    ) -> anyhow::Result<crate::version::VersionContext> {
        let version_line = crate::edit::toml::query(
            version_file_content,
            &self
                .base
                .settings
                .selector
                .clone()
                .ok_or(anyhow::anyhow!("Selector is required for a TOML project",))?,
        )?;
        Ok(VersionContext::new(
            version_line.value.to_version(),
            version_line.line_number,
        ))
    }
}
