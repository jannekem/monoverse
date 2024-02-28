use anyhow::Result;
use config::Config;
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{dependents::DependentType, projects::ProjectType};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub projects: HashMap<String, ProjectSettings>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectSettings {
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    #[serde(default, rename = "path")]
    pub project_path: PathBuf,
    pub manifest_path: Option<PathBuf>,
    pub selector: Option<String>,
    pub dependents: Option<Vec<DependentSettings>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DependentSettings {
    #[serde(rename = "type")]
    pub dependent_type: DependentType,
    #[serde(default, rename = "path")]
    pub dependent_path: PathBuf,
    pub selector: Option<String>,
    pub replace: Option<String>,
}

impl Settings {
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        let config_path = repo_path.as_ref().join("monoverse");
        let settings = Config::builder()
            .add_source(config::File::with_name(config_path.to_str().unwrap()))
            .build()?;
        let deserialized: Self = settings.try_deserialize()?;
        Ok(deserialized)
    }

    pub fn project_settings(&self, project_name: &str) -> Result<&ProjectSettings> {
        self.projects
            .get(project_name)
            .ok_or_else(|| anyhow::anyhow!("No project found with name: {}", project_name))
    }
}

impl ProjectSettings {
    /// Return the path to the version file for each project type
    ///
    /// If the project path is defined as "." then it is stripped
    /// from the joined path so that the version file path works
    /// with the git2 library.
    ///
    /// If the manifest path is defined, then it is used instead
    pub fn get_manifest_file_path(&self) -> Result<PathBuf> {
        if let Some(manifest_path) = &self.manifest_path {
            return Ok(manifest_path.to_path_buf());
        }
        let path = match self.project_type {
            ProjectType::Helm => self.project_path.join("Chart.yaml"),
            ProjectType::Node => self.project_path.join("package.json"),
            ProjectType::Rust => self.project_path.join("Cargo.toml"),
            ProjectType::Toml => Err(anyhow::anyhow!("TOML project requires a manifest path"))?,
            ProjectType::Yaml => Err(anyhow::anyhow!("YAML project requires a manifest path"))?,
        };
        Ok(path.strip_prefix("./").unwrap_or(&path).to_path_buf())
    }
}
