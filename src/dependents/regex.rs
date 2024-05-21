use anyhow::Result;
use regex::RegexBuilder;
use std::path::PathBuf;

use crate::{settings::DependentSettings, version::Version};

use super::Dependent;

#[derive(Debug)]
pub struct RegexDependent {
    pub settings: DependentSettings,
    pub repo_path: PathBuf,
}

impl Dependent for RegexDependent {
    fn update_version(&self, version: &Version) -> Result<()> {
        let file_content = crate::io::read_file(&self.settings.dependent_path, &self.repo_path)?;
        let new_file_content = update_regex(&file_content, version, &self.settings)?;
        crate::io::write_file(
            &self.settings.dependent_path,
            &self.repo_path,
            new_file_content.as_str(),
        )?;
        Ok(())
    }

    fn get_file_path(&self) -> PathBuf {
        self.settings.dependent_path.clone()
    }
}

fn update_regex(
    file_content: &str,
    version: &Version,
    settings: &DependentSettings,
) -> Result<String> {
    let selector = settings
        .selector
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Selector is required for regex dependent"))?;
    let replace = &settings
        .replace
        .clone()
        .unwrap_or(version.to_string())
        .replace("{{version}}", &format!("{}", version));
    let pattern = RegexBuilder::new(selector).multi_line(true).build()?;
    let new_file_content = pattern.replace(file_content, replace.as_str());
    Ok(new_file_content.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dependents::DependentType;
    use crate::version::ToVersion;

    #[test]
    fn test_update_regex() {
        let file_content = r#"version = "0.1.0""#;
        let version = "0.2.0".to_version();
        let settings = DependentSettings {
            dependent_type: DependentType::Regex,
            dependent_path: PathBuf::from("Cargo.toml"),
            selector: Some(r#"(\d+\.\d+\.\d+)"#.to_string()),
            replace: None,
        };
        let new_file_content = update_regex(file_content, &version, &settings).unwrap();
        assert_eq!(new_file_content, r#"version = "0.2.0""#);
    }

    #[test]
    fn test_update_regex_with_replace() {
        let file_content = r#"version = "0.1.0""#;
        let version = "0.2.0".to_version();
        let settings = DependentSettings {
            dependent_type: DependentType::Regex,
            dependent_path: PathBuf::from("Cargo.toml"),
            selector: Some(r#"version = "(.*)""#.to_string()),
            replace: Some(r#"version = "{{version}}""#.to_string()),
        };
        let new_file_content = update_regex(file_content, &version, &settings).unwrap();
        assert_eq!(new_file_content, r#"version = "0.2.0""#);
    }

    #[test]
    fn test_update_regex_multi_line() {
        let file_content = r#"
[package]
version = "0.1.0"
"#;
        let version = "0.2.0".to_version();
        let settings = DependentSettings {
            dependent_type: DependentType::Regex,
            dependent_path: PathBuf::from("Cargo.toml"),
            selector: Some(r#"version = "(.*)""#.to_string()),
            replace: Some(r#"version = "{{version}}""#.to_string()),
        };
        let new_file_content = update_regex(file_content, &version, &settings).unwrap();
        assert_eq!(
            new_file_content,
            r#"
[package]
version = "0.2.0"
"#
        );
    }
}
