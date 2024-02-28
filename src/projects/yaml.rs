use std::path::PathBuf;

use crate::{
    settings::ProjectSettings,
    version::{ToVersion, VersionContext},
};

pub struct YamlProject {
    base: super::BaseProjectFile,
}

impl YamlProject {
    pub fn new(settings: ProjectSettings, repo_path: PathBuf) -> Self {
        Self {
            base: super::BaseProjectFile {
                settings,
                repo_path,
            },
        }
    }
}

impl super::ProjectFile for YamlProject {
    fn base(&self) -> &super::BaseProjectFile {
        &self.base
    }

    fn update_version(
        &self,
        version_file_content: &str,
        version_context: &super::VersionContext,
    ) -> anyhow::Result<String> {
        crate::edit::yaml::edit(
            version_file_content,
            &self
                .base
                .settings
                .selector
                .clone()
                .ok_or(anyhow::anyhow!("Selector is required for a YAML project",))?,
            &version_context.next_version.to_string(),
        )
    }

    fn version_context(
        &self,
        version_file_content: &str,
    ) -> anyhow::Result<crate::version::VersionContext> {
        let version_line = crate::edit::yaml::query(
            version_file_content,
            &self
                .base
                .settings
                .selector
                .clone()
                .ok_or(anyhow::anyhow!("Selector is required for a YAML project",))?,
        )?;
        Ok(VersionContext::new(
            version_line.value.to_version(),
            version_line.line_number,
        ))
    }
}
