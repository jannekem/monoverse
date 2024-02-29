use anyhow::Result;
use std::path::PathBuf;

use crate::version::Version;

use super::Dependent;

/// General dependent for YAML files
#[derive(Debug)]
pub struct YamlDepedent {
    pub file_path: PathBuf,
    pub selector: String,
    pub repo_path: PathBuf,
}

impl Dependent for YamlDepedent {
    fn update_version(&self, version: &Version) -> Result<()> {
        let file_content = crate::io::read_file(&self.file_path, &self.repo_path)?;
        let new_file_content =
            crate::edit::yaml::edit(&file_content, &self.selector, &version.to_string())?;
        crate::io::write_file(&self.file_path, &self.repo_path, new_file_content.as_str())?;
        Ok(())
    }
}
