use anyhow::Result;
use std::{fmt::Debug, path::PathBuf};

use crate::{settings::DependentSettings, version::Version};
use serde::Deserialize;

mod regex;
mod toml;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DependentType {
    Regex,
    Toml,
}

pub fn get_dependent(
    dependent_settings: &DependentSettings,
    repo_path: PathBuf,
) -> Result<Box<dyn Dependent>> {
    match dependent_settings.dependent_type {
        DependentType::Regex => Ok(Box::new(regex::RegexDependent {
            settings: dependent_settings.clone(),
            repo_path,
        })),
        DependentType::Toml => Ok(Box::new(toml::TomlDependent {
            file_path: dependent_settings.dependent_path.clone(),
            selector: dependent_settings
                .selector
                .clone()
                .ok_or_else(|| anyhow::anyhow!("Selector is required for TOML dependent"))?,
            repo_path,
        })),
    }
}

pub trait Dependent: Debug {
    fn update_version(&self, version: &Version) -> Result<()>;
}
