use anyhow::Result;
use toml_edit::{value, Document, Item};

use super::LineContext;

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
pub fn edit(file_content: &str, selector: &str, new_value: &str) -> Result<String> {
    let mut doc = file_content.parse::<Document>()?;
    set_value(&mut doc, selector, new_value)?;
    Ok(doc.to_string())
}

/// Query a TOML file for a value
pub fn query(file_content: &str, selector: &str) -> Result<LineContext> {
    log::info!("Querying TOML file for selector: {}", selector);
    let mut doc = file_content.parse::<Document>()?;
    let value = {
        let value = get_value(&doc, selector)?.as_str().ok_or(anyhow::anyhow!(
            "Failed to parse value from TOML file for selector: {}",
            selector
        ))?;
        value.to_string()
    };
    let detector_value = "<<<<<<<monoverse-detector>>>>>>";
    set_value(&mut doc, selector, detector_value)?;
    let line_number = doc
        .to_string()
        .lines()
        .position(|line| line.contains(detector_value))
        .map(|pos| pos + 1)
        .ok_or_else(|| anyhow::anyhow!("Failed to get line number in TOML file"))?;
    Ok(LineContext { line_number, value })
}

fn get_value<'a>(doc: &'a Document, selector: &str) -> Result<&'a Item> {
    let keys = selector.split('.').collect::<Vec<_>>();
    if keys.len() == 1 {
        Ok(&doc[keys[0]])
    } else {
        let mut item = &doc[keys[0]];
        assert_not_array_of_tables(item)?;
        for key in &keys[1..keys.len() - 1] {
            item = &item[key];
            assert_not_array_of_tables(item)?;
        }
        Ok(&item[keys[keys.len() - 1]])
    }
}

fn set_value(doc: &mut Document, selector: &str, new_value: &str) -> Result<()> {
    let keys = selector.split('.').collect::<Vec<_>>();
    if keys.len() == 1 {
        doc[keys[0]] = value(new_value);
    } else {
        let mut item = &mut doc[keys[0]];
        assert_not_array_of_tables(item)?;
        for key in &keys[1..keys.len() - 1] {
            item = &mut item[key];
            assert_not_array_of_tables(item)?;
        }
        item[keys[keys.len() - 1]] = value(new_value);
    }
    Ok(())
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
    fn test_edit_top_level() {
        let file_content = r#"version = "1.0.111""#;
        let version = "1.0.112".to_version();
        let selector = "version";
        let new_file_content = edit(file_content, selector, &version.to_string()).unwrap();
        assert_eq!(
            new_file_content,
            "version = \"1.0.112\"\n" // Note toml_edit adds a newline
        );
    }

    #[test]
    fn test_edit_cargo_version() {
        let file_content = r#"[dependencies]
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
"#;
        let version = "1.0.112".to_version();
        let selector = "dependencies.serde_json";
        let new_file_content = edit(file_content, selector, &version.to_string()).unwrap();
        assert_eq!(
            new_file_content,
            r#"[dependencies]
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.112"
"#
        );
    }

    #[test]
    fn test_edit_cargo_version_dict() {
        let file_content = r#"[dependencies]
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
"#;
        let version = "1.0.196".to_version();
        let selector = "dependencies.serde.version";
        let new_file_content = edit(file_content, selector, &version.to_string()).unwrap();
        assert_eq!(
            new_file_content,
            r#"[dependencies]
serde = { version = "1.0.196", features = ["derive"] }
serde_json = "1.0.111"
"#
        );
    }

    #[test]
    fn test_edit_array_of_tables() {
        let file_content = r#"[[dependencies]]
        "#;
        let version = "1.0.196".to_version();
        let selector = "dependencies.version";
        let result = edit(file_content, selector, &version.to_string());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Array of tables not supported for TOML dependent"));
    }

    #[test]
    fn test_query_top_level() {
        let file_content = r#"version = "1.0.111""#;
        let selector = "version";
        let line_context = query(file_content, selector).unwrap();
        assert_eq!(line_context.value, "1.0.111");
        assert_eq!(line_context.line_number, 1);
    }

    #[test]
    fn test_query_cargo_version() {
        let file_content = r#"[dependencies]
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
"#;
        let selector = "dependencies.serde_json";
        let line_context = query(file_content, selector).unwrap();
        assert_eq!(line_context.value, "1.0.111");
        assert_eq!(line_context.line_number, 3);
    }

    #[test]
    fn test_query_cargo_version_dict() {
        let file_content = r#"[dependencies]
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
"#;
        let selector = "dependencies.serde.version";
        let line_context = query(file_content, selector).unwrap();
        assert_eq!(line_context.value, "1.0.195");
        assert_eq!(line_context.line_number, 2);
    }
}
