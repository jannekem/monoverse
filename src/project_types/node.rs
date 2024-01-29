use std::path::PathBuf;

use anyhow::Result;
use regex::Regex;
use serde_json::Value;

use crate::{
    settings::ProjectSettings,
    version::{ToVersion, Version, VersionContext},
};

pub struct NodeProject {
    base: super::BaseProjectFile,
}

impl NodeProject {
    pub fn new(settings: ProjectSettings, repo_path: PathBuf) -> Self {
        Self {
            base: super::BaseProjectFile {
                settings,
                repo_path,
            },
        }
    }
}

impl super::ProjectFile for NodeProject {
    fn base(&self) -> &super::BaseProjectFile {
        &self.base
    }

    /// Bump version in package.json
    ///
    /// Use serde_json to parse the package.json file and find
    /// the version field. Parse the version field into a Version
    /// struct and bump the version. Write the new version back
    /// to the package.json file.
    ///
    /// In order to preserve formatting of the package.json file,
    /// we read the file into a string but don't use serde_json
    /// to write the file back. Instead, we use a regular
    /// expression to replace the version field with the new
    /// version and write the string back to the file.
    fn bump_version(&self, version_file_content: &str, current_version: Version) -> Result<String> {
        let pattern = Regex::new(&format!(r#""version"\s*:\s*"{}""#, current_version))?;
        let new_package_json = pattern.replace(
            &version_file_content,
            format!(r#""version": "{}""#, current_version.bump()),
        );
        Ok(new_package_json.into_owned())
    }

    fn get_current_version_context(&self, version_file_content: &str) -> Result<VersionContext> {
        let value: Value = serde_json::from_str(&version_file_content)?;
        let current_version = value["version"]
            .as_str()
            .ok_or(anyhow::anyhow!(
                "Failed to parse version from package.json: {:?}",
                self.base.settings.get_manifest_file_path()
            ))?
            .to_version();
        let pattern = Regex::new(&format!(r#""version"\s*:\s*"{}""#, current_version))?;
        let line_number = version_file_content
            .lines()
            .enumerate()
            .find(|(_, line)| pattern.is_match(line))
            .map(|(line_number, _)| line_number + 1)
            .ok_or(anyhow::anyhow!(
                "Failed to find version line number in package.json: {:?}",
                self.base.settings.get_manifest_file_path()
            ))?;
        log::info!("Version line number: {}", line_number);
        Ok(VersionContext {
            version: current_version,
            line_number,
        })
    }
}
