use std::path::PathBuf;

use anyhow::Result;

use crate::{
    edit::versionfile,
    settings::ProjectSettings,
    version::{ToVersion, VersionContext},
};

pub struct VersionfileProject {
    base: super::BaseProjectFile,
}

impl VersionfileProject {
    pub fn new(settings: ProjectSettings, repo_path: PathBuf) -> Self {
        Self {
            base: super::BaseProjectFile {
                settings,
                repo_path,
            },
        }
    }
}

impl super::ProjectFile for VersionfileProject {
    fn base(&self) -> &super::BaseProjectFile {
        &self.base
    }

    fn update_version(
        &self,
        _version_file_content: &str,
        version_context: &crate::version::VersionContext,
    ) -> anyhow::Result<String> {
        versionfile::edit(&version_context.next_version.to_string())
    }

    fn version_context(
        &self,
        version_file_content: &str,
    ) -> Result<crate::version::VersionContext> {
        let version_line = versionfile::query(version_file_content)?;
        Ok(VersionContext::new(
            version_line.value.to_version(),
            version_line.line_number,
        ))
    }
}
