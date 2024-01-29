use anyhow::Result;
use std::path::PathBuf;
use toml_edit::{value, Document};

use crate::{
    settings::ProjectSettings,
    version::{ToVersion, Version},
};

pub struct RustProject {
    base: super::BaseProjectFile,
}

impl RustProject {
    pub fn new(settings: ProjectSettings, repo_path: PathBuf) -> Self {
        Self {
            base: super::BaseProjectFile {
                settings,
                repo_path,
            },
        }
    }
}

impl super::ProjectFile for RustProject {
    fn base(&self) -> &super::BaseProjectFile {
        &self.base
    }

    fn get_current_version_context(
        &self,
        version_file_content: &str,
    ) -> anyhow::Result<crate::version::VersionContext> {
        let doc = version_file_content.parse::<Document>()?;
        let current_version = doc["package"]["version"]
            .as_str()
            .ok_or(anyhow::anyhow!(
                "Failed to parse version from Cargo.toml: {:?}",
                self.base.settings.get_manifest_file_path()
            ))?
            .to_version();
        let pattern = regex::Regex::new(&format!(r#"^version\s*=\s*"{}""#, current_version))?;
        let line_number = version_file_content
            .lines()
            .enumerate()
            .find(|(_, line)| pattern.is_match(line))
            .map(|(line_number, _)| line_number + 1)
            .ok_or(anyhow::anyhow!(
                "Failed to find version in Cargo.toml: {:?}",
                self.base.settings.get_manifest_file_path()
            ))?;
        log::info!(
            "Found version {} in Cargo.toml at line {}",
            current_version,
            line_number
        );
        Ok(crate::version::VersionContext {
            version: current_version,
            line_number,
        })
    }

    /// Bump version in Cargo.toml
    ///
    /// toml_edit preserves the formatting of the original file
    fn bump_version(&self, version_file_content: &str, current_version: Version) -> Result<String> {
        let mut doc = version_file_content.parse::<Document>()?;
        doc["package"]["version"] = value(current_version.bump().to_string());
        Ok(doc.to_string())
    }
}
