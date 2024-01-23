use std::path::PathBuf;

use anyhow::{Context, Result};
use regex::Regex;
use serde_json::Value;

pub struct NodeProject {
    path: PathBuf,
    repo_path: PathBuf,
}

impl NodeProject {
    pub fn new(path: PathBuf, repo_path: PathBuf) -> Self {
        Self { path, repo_path }
    }
}

impl super::ProjectFile for NodeProject {
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
    fn bump_version(&self) -> Result<()> {
        let package_json_path = self.repo_path.join(&self.path).join("package.json");
        let package_json_content = std::fs::read_to_string(&package_json_path)
            .with_context(|| format!("Could not read file at: {:?}", package_json_path))?;
        let value: Value = serde_json::from_str(&package_json_content)?;
        let current_version = value["version"].as_str().ok_or(anyhow::anyhow!(
            "Failed to parse version from package.json: {:?}",
            package_json_path
        ))?;
        let new_version = crate::version::Version::parse(&current_version).bump();
        let pattern = Regex::new(&format!(r#""version"\s*:\s*"{}""#, current_version))?;
        let new_package_json = pattern.replace(
            &package_json_content,
            format!(r#""version": "{}""#, new_version),
        );
        std::fs::write(&package_json_path, new_package_json.as_bytes())?;
        Ok(())
    }
}
