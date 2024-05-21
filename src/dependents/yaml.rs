use anyhow::Result;
use std::path::PathBuf;

use crate::{settings::DependentSettings, version::Version};

use super::Dependent;

/// General dependent for YAML files
#[derive(Debug)]
pub struct YamlDepedent {
    pub settings: DependentSettings,
    pub repo_path: PathBuf,
}

impl Dependent for YamlDepedent {
    fn update_version(&self, version: &Version) -> Result<()> {
        let file_path = &self.settings.dependent_path;
        let repo_path = &self.repo_path;
        let selector = self
            .settings
            .selector
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Selector is required for YAML dependent"))?;
        let file_content = crate::io::read_file(file_path, repo_path)?;
        let new_file_content =
            crate::edit::yaml::edit(&file_content, &selector, &version.to_string())?;
        crate::io::write_file(file_path, repo_path, new_file_content.as_str())?;
        Ok(())
    }

    fn get_file_path(&self) -> PathBuf {
        self.settings.dependent_path.clone()
    }
}
