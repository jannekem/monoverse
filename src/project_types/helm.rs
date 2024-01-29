use anyhow::Result;
use regex::RegexBuilder;
use serde_yaml::Value;
use std::path::PathBuf;

use crate::{
    settings::ProjectSettings,
    version::{ToVersion, VersionContext},
};

pub struct HelmProject {
    base: super::BaseProjectFile,
}

impl HelmProject {
    pub fn new(settings: ProjectSettings, repo_path: PathBuf) -> Self {
        Self {
            base: super::BaseProjectFile {
                settings,
                repo_path,
            },
        }
    }
}

impl super::ProjectFile for HelmProject {
    fn base(&self) -> &super::BaseProjectFile {
        &self.base
    }

    fn get_current_version_context(&self, version_file_content: &str) -> Result<VersionContext> {
        let value: Value = serde_yaml::from_str(version_file_content)?;
        let version = value["appVersion"]
            .as_str()
            .ok_or(anyhow::anyhow!(
                "Failed to parse version from Chart.yaml: {:?}",
                self.base.settings.get_manifest_file_path()
            ))?
            .to_version();
        let pattern = regex::Regex::new(&format!(r#"^appVersion:\s*"?{}"?"#, version))?;
        let line_number = version_file_content
            .lines()
            .enumerate()
            .find(|(_, line)| pattern.is_match(line))
            .map(|(line_number, _)| line_number + 1)
            .ok_or(anyhow::anyhow!(
                "Failed to find version in Chart.yaml: {:?}",
                self.base.settings.get_manifest_file_path()
            ))?;
        log::info!(
            "Found version {} in Chart.yaml at line {}",
            version,
            line_number
        );
        Ok(VersionContext {
            version,
            line_number,
        })
    }

    fn bump_version(
        &self,
        version_file_content: &str,
        current_version: crate::version::Version,
    ) -> Result<String> {
        let value: Value = serde_yaml::from_str(version_file_content)?;
        let chart_version = value["version"]
            .as_str()
            .ok_or(anyhow::anyhow!(
                "Failed to parse version from Chart.yaml: {:?}",
                self.base.settings.get_manifest_file_path()
            ))?
            .to_version();
        let app_version_pattern =
            RegexBuilder::new(&format!(r#"^appVersion:\s*"?{}"?"#, current_version))
                .multi_line(true)
                .build()?;
        let chart_version_pattern =
            RegexBuilder::new(&format!(r#"^version:\s*"?{}"?"#, chart_version))
                .multi_line(true)
                .build()?;
        let result = app_version_pattern.replace(
            &version_file_content,
            format!(r#"appVersion: "{}""#, current_version.bump()),
        );
        let result = chart_version_pattern.replace(
            &result,
            format!(r#"version: {}"#, chart_version.bump_patch()),
        );
        Ok(result.into_owned())
    }
}
