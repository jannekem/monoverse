use anyhow::Result;
use config::Config;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

use crate::project_types::ProjectType;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub projects: HashMap<String, AppSettings>,
}

#[derive(Deserialize, Debug)]
pub struct AppSettings {
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    pub path: String,
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
