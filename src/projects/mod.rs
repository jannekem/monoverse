use std::path::PathBuf;

use anyhow::Result;
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
pub mod toml;
pub mod versionfile;
pub mod yaml;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Helm,
    Node,
    Rust,
    Toml,
    Versionfile,
    Yaml,
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
        ProjectType::Toml => Box::new(toml::TomlProject::new(settings, repo_path)),
        ProjectType::Versionfile => {
            Box::new(versionfile::VersionfileProject::new(settings, repo_path))
        }
        ProjectType::Yaml => Box::new(yaml::YamlProject::new(settings, repo_path)),
    }
}

pub trait ProjectFile {
    fn base(&self) -> &BaseProjectFile;
    fn update_version(
        &self,
        version_file_content: &str,
        version_context: &VersionContext,
    ) -> Result<String>;

    /// Return the current version and the line number of the version field
    fn version_context(&self, version_file_content: &str) -> Result<VersionContext>;

    /// Release project
    ///
    /// If the project has changed since the last release, update the version
    /// and write it to the manifest file. Return the new version.
    ///
    /// If the project has not changed since the last release, return None.
    fn release(&self, repo: &Repository, force: bool) -> Result<Option<Version>> {
        let version_file_path = self.get_manifest_file_path()?;
        let version_file_status = repo.status_file(&version_file_path)?;
        if version_file_status.is_wt_modified() || version_file_status.is_index_modified() {
            return Err(anyhow::anyhow!(
                "The version file '{}' has been modified. Please stash or commit your changes before releasing.",
                version_file_path.display()
            ));
        }
        let version_file_content =
            crate::io::read_file(&version_file_path, &self.base().repo_path)?;
        let version_context = self.version_context(&version_file_content)?;

        let do_release = match force {
            true => true,
            false => {
                let commit_id = git::get_commit_id_for_line(
                    repo,
                    &version_file_path,
                    version_context.line_number,
                )?;
                git::has_path_changed_since(repo, &self.base().settings.project_path, commit_id)?
            }
        };
        match do_release {
            true => {
                log::info!("There are changes to the project.");
                let new_version_file =
                    self.update_version(&version_file_content, &version_context)?;
                crate::io::write_file(
                    &version_file_path,
                    &self.base().repo_path,
                    new_version_file.as_str(),
                )?;
                Ok(Some(version_context.next_version))
            }
            false => {
                log::info!("There are no changes to the project.");
                Ok(None)
            }
        }
    }

    /// Print the next version for the project
    fn print_next_version(&self) -> Result<()> {
        let version_file_path = self.get_manifest_file_path()?;
        let version_file_content =
            crate::io::read_file(&version_file_path, &self.base().repo_path)?;
        let version_context = self.version_context(&version_file_content)?;
        println!("{}", version_context.version.bump());
        Ok(())
    }

    /// Get the path to the manifest file
    fn get_manifest_file_path(&self) -> Result<PathBuf> {
        self.base().settings.get_manifest_file_path()
    }
}
