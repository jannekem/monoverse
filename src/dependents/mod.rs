use anyhow::Result;
use std::{fmt::Debug, path::PathBuf};

use crate::{settings::DependentSettings, version::Version};
use serde::Deserialize;

mod helm;
mod regex;
mod toml;
mod yaml;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DependentType {
    Helm,
    Regex,
    Toml,
    Yaml,
}

pub fn get_dependent(
    dependent_settings: &DependentSettings,
    repo_path: PathBuf,
) -> Result<Box<dyn Dependent>> {
    match dependent_settings.dependent_type {
        DependentType::Helm => Ok(Box::new(helm::HelmDependent {
            settings: dependent_settings.clone(),
            repo_path,
        })),
        DependentType::Regex => Ok(Box::new(regex::RegexDependent {
            settings: dependent_settings.clone(),
            repo_path,
        })),
        DependentType::Toml => Ok(Box::new(toml::TomlDependent {
            settings: dependent_settings.clone(),
            repo_path,
        })),
        DependentType::Yaml => Ok(Box::new(yaml::YamlDepedent {
            settings: dependent_settings.clone(),
            repo_path,
        })),
    }
}

pub trait Dependent: Debug {
    /// Update the dependent and return the list of files it modified.
    fn update_version(
        &self,
        version: &Version,
        options: &DependentUpdateOptions,
    ) -> Result<Vec<PathBuf>>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DependentUpdateOptions {
    pub helm_dependency_update: bool,
}
