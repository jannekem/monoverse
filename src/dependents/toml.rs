use anyhow::Result;
use std::path::PathBuf;

use crate::{settings::DependentSettings, version::Version};

use super::Dependent;

/// General dependent for TOML files
#[derive(Debug)]
pub struct TomlDependent {
    pub settings: DependentSettings,
    pub repo_path: PathBuf,
}

impl Dependent for TomlDependent {
    fn update_version(
        &self,
        version: &Version,
        _options: &super::DependentUpdateOptions,
    ) -> Result<Vec<PathBuf>> {
        let file_path = &self.settings.dependent_path;
        let repo_path = &self.repo_path;
        let selector = self
            .settings
            .selector
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Selector is required for TOML dependent"))?;
        let file_content = crate::io::read_file(file_path, repo_path)?;
        let new_file_content =
            crate::edit::toml::edit(&file_content, &selector, &version.to_string())?;
        crate::io::write_file(file_path, repo_path, new_file_content.as_str())?;
        Ok(vec![self.settings.dependent_path.clone()])
    }
}
