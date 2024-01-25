use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use git2::Repository;
use serde::Deserialize;

use crate::{
    git,
    settings::AppSettings,
    version::{Version, VersionContext},
};

pub mod node;
pub mod rust;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Node,
    Rust,
}

pub struct BaseProjectFile {
    pub settings: AppSettings,
    pub repo_path: PathBuf,
}

pub trait ProjectFile {
    fn base(&self) -> &BaseProjectFile;
    fn bump_version(&self, version_file_content: &str, current_version: Version) -> Result<String>;

    /// Return the current version and the line number of the version field
    fn get_current_version_context(&self, version_file_content: &str) -> Result<VersionContext>;

    fn release(&self, repo: &Repository) -> Result<()> {
        let version_file_path = self.base().settings.get_version_file_path();
        let version_file_content = self.read_file(&version_file_path)?;
        let version_context = self.get_current_version_context(&version_file_content)?;

        let commit_id =
            git::get_commit_id_for_line(&repo, &version_file_path, version_context.line_number)?;
        log::info!("Commit ID: {}", commit_id);
        let has_changed =
            git::has_path_changed_since(&repo, &self.base().settings.path, commit_id)?;
        log::info!("Has changed: {}", has_changed);
        if has_changed {
            log::info!("There are changes to the project.");
            let new_version_file =
                self.bump_version(&version_file_content, version_context.version)?;
            self.write_file(&version_file_path, new_version_file.as_str())?;
        }
        Ok(())
    }

    fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let path = self.base().repo_path.join(path.as_ref());
        let version_file_content = std::fs::read_to_string(&path)
            .with_context(|| format!("Could not read file at: {:}", path.display()))?;
        Ok(version_file_content)
    }

    fn write_file<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<()> {
        let path = self.base().repo_path.join(path.as_ref());
        std::fs::write(&path, content)?;
        Ok(())
    }
}
