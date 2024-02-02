use anyhow::Result;
use std::path::PathBuf;
use toml_edit::{value, Document, Item};

use crate::version::Version;

use super::Dependent;

/// General dependent for TOML files
#[derive(Debug)]
pub struct TomlDependent {
    pub file_path: PathBuf,
    pub selector: String,
    pub repo_path: PathBuf,
}

impl Dependent for TomlDependent {
    fn update_version(&self, version: &Version) -> Result<()> {
        let file_content = crate::io::read_file(&self.file_path, &self.repo_path)?;
        let new_file_content = update_toml(&file_content, version, &self.selector)?;
        crate::io::write_file(&self.file_path, &self.repo_path, new_file_content.as_str())?;
        Ok(())
    }
}

/// Update a TOML file with a new version
///
/// Arguments:
/// * `file_content` - The content of the TOML file
/// * `version` - The new version
/// * `selector` - The selector for the version field
///
/// The selector is a dot-separated list of keys to the version field.
/// For example, if the version field is at the top level of the TOML
/// file, the selector is "version". If the version field is in a
/// table called "package", the selector is "package.version".
/// If the version field is in a table called "package" and the
/// table is in an array called "dependencies", the selector is
/// "dependencies.package.version".
///
/// Arrays of tables are not supported.
fn update_toml(file_content: &str, version: &Version, selector: &str) -> Result<String> {
    let mut doc = file_content.parse::<Document>()?;
    let keys = selector.split('.').collect::<Vec<_>>();
    if keys.len() == 1 {
        doc[&keys[0]] = value(version.to_string());
    } else {
        let mut item = &mut doc[keys[0]];
        assert_not_array_of_tables(item)?;
        for key in &keys[1..keys.len() - 1] {
            item = &mut item[key];
            assert_not_array_of_tables(item)?;
        }
        item[&keys[keys.len() - 1]] = value(version.to_string());
    }
    Ok(doc.to_string())
}

fn assert_not_array_of_tables(item: &Item) -> Result<()> {
    if item.is_array_of_tables() {
        return Err(anyhow::anyhow!(
            "Array of tables not supported for TOML dependent"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::version::ToVersion;

    use super::*;

    #[test]
    fn test_update_toml_top_level() {
        let file_content = r#"version = "1.0.111""#;
        let version = "1.0.112".to_version();
        let selector = "version";
        let new_file_content = update_toml(file_content, &version, selector).unwrap();
        assert_eq!(
            new_file_content,
            "version = \"1.0.112\"\n" // Note toml_edit adds a newline
        );
    }

    #[test]
    fn test_update_toml_cargo_version() {
        let file_content = r#"[dependencies]
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
"#;
        let version = "1.0.112".to_version();
        let selector = "dependencies.serde_json";
        let new_file_content = update_toml(file_content, &version, selector).unwrap();
        assert_eq!(
            new_file_content,
            r#"[dependencies]
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.112"
"#
        );
    }

    #[test]
    fn test_update_toml_cargo_version_dict() {
        let file_content = r#"[dependencies]
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
"#;
        let version = "1.0.196".to_version();
        let selector = "dependencies.serde.version";
        let new_file_content = update_toml(file_content, &version, selector).unwrap();
        assert_eq!(
            new_file_content,
            r#"[dependencies]
serde = { version = "1.0.196", features = ["derive"] }
serde_json = "1.0.111"
"#
        );
    }

    #[test]
    fn test_update_toml_array_of_tables() {
        let file_content = r#"[[dependencies]]
        "#;
        let version = "1.0.196".to_version();
        let selector = "dependencies.version";
        let result = update_toml(file_content, &version, selector);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Array of tables not supported for TOML dependent"));
    }
}
