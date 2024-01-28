use anyhow::Result;
use config::Config;
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::project_types::ProjectType;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub projects: HashMap<String, AppSettings>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppSettings {
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    #[serde(default)]
    pub path: PathBuf,
    pub manifest_path: Option<PathBuf>,
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
}

impl AppSettings {
    /// Return the path to the version file for each project type
    ///
    /// If the project path is defined as "." then it is stripped
    /// from the joined path so that the version file path works
    /// with the git2 library.
    ///
    /// If the manifest path is defined, then it is used instead
    pub fn get_manifest_file_path(&self) -> PathBuf {
        if let Some(manifest_path) = &self.manifest_path {
            return manifest_path.to_path_buf();
        }
        let path = match self.project_type {
            ProjectType::Helm => self.path.join("Chart.yaml"),
            ProjectType::Node => self.path.join("package.json"),
            ProjectType::Rust => self.path.join("Cargo.toml"),
        };
        path.strip_prefix("./").unwrap_or(&path).to_path_buf()
    }
}
