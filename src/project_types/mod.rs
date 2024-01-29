use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use git2::Repository;
use serde::Deserialize;

use crate::{
    git,
    settings::ProjectSettings,
    version::{Version, VersionContext},
};

pub mod helm;
pub mod node;
pub mod rust;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Helm,
    Node,
    Rust,
}

pub struct BaseProjectFile {
    pub settings: ProjectSettings,
    pub repo_path: PathBuf,
}

pub fn get_project_file(settings: ProjectSettings, repo_path: PathBuf) -> Box<dyn ProjectFile> {
    match settings.project_type {
        ProjectType::Helm => Box::new(helm::HelmProject::new(settings, repo_path)),
        ProjectType::Node => Box::new(node::NodeProject::new(settings, repo_path)),
        ProjectType::Rust => Box::new(rust::RustProject::new(settings, repo_path)),
    }
}

pub trait ProjectFile {
    fn base(&self) -> &BaseProjectFile;
    fn bump_version(&self, version_file_content: &str, current_version: Version) -> Result<String>;

    /// Return the current version and the line number of the version field
    fn get_current_version_context(&self, version_file_content: &str) -> Result<VersionContext>;

    fn release(&self, repo: &Repository) -> Result<()> {
        let version_file_path = self.base().settings.get_manifest_file_path();
        let version_file_content = read_file(&version_file_path, &self.base().repo_path)?;
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
            write_file(
                &version_file_path,
                &self.base().repo_path,
                new_version_file.as_str(),
            )?;
        }
        Ok(())
    }

    fn print_next_version(&self) -> Result<()> {
        let version_file_path = self.base().settings.get_manifest_file_path();
        let version_file_content = read_file(&version_file_path, &self.base().repo_path)?;
        let version_context = self.get_current_version_context(&version_file_content)?;
        println!("{}", version_context.version.bump());
        Ok(())
    }
}

fn read_file<P: AsRef<Path>>(path: P, repo_path: P) -> Result<String> {
    let path = repo_path.as_ref().join(path.as_ref());
    let version_file_content = std::fs::read_to_string(&path)
        .with_context(|| format!("Could not read file at: {:}", path.display()))?;
    Ok(version_file_content)
}

fn write_file<P: AsRef<Path>>(path: P, repo_path: P, content: &str) -> Result<()> {
    let path = repo_path.as_ref().join(path.as_ref());
    std::fs::write(&path, content)?;
    Ok(())
}
