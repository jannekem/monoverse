use anyhow::Result;
use toml_edit::{value, DocumentMut, Item};

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
    let mut doc = file_content.parse::<DocumentMut>()?;
    set_value(&mut doc, selector, new_value)?;
    Ok(doc.to_string())
}

/// Query a TOML file for a value
pub fn query(file_content: &str, selector: &str) -> Result<LineContext> {
    log::info!("Querying TOML file for selector: {}", selector);
    let mut doc = file_content.parse::<DocumentMut>()?;
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

fn split_selector(selector: &str) -> Result<Vec<&str>> {
    if selector.is_empty() {
        return Err(anyhow::anyhow!("Selector cannot be empty"));
    }
    if selector.starts_with('.') || selector.ends_with('.') || selector.contains("..") {
        return Err(anyhow::anyhow!("Invalid selector: {}", selector));
    }
    Ok(selector.split('.').collect())
}

fn get_value<'a>(doc: &'a DocumentMut, selector: &str) -> Result<&'a Item> {
    let keys = split_selector(selector)?;
    let mut processed_keys = Vec::new();
    let mut item = doc
        .get(keys[0])
        .ok_or_else(|| anyhow::anyhow!("Key not found: {}", keys[0]))?;
    processed_keys.push(keys[0]);
    assert_not_array_of_tables(item)?;
    if keys.len() == 1 {
        return Ok(item);
    }
    for key in &keys[1..keys.len() - 1] {
        let table = item.as_table_like().ok_or_else(|| {
            anyhow::anyhow!("Value for '{}' is not a table", processed_keys.join("."))
        })?;
        let next_path = format!("{}.{}", processed_keys.join("."), key);
        item = table
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("Key '{}' not found", next_path))?;
        processed_keys.push(*key);
        assert_not_array_of_tables(item)?;
    }
    let table = item.as_table_like().ok_or_else(|| {
        anyhow::anyhow!("Value for '{}' is not a table", processed_keys.join("."))
    })?;
    let last_key = keys[keys.len() - 1];
    let last_path = format!("{}.{}", processed_keys.join("."), last_key);
    let item = table
        .get(last_key)
        .ok_or_else(|| anyhow::anyhow!("Key '{}' not found", last_path))?;
    Ok(item)
}

// Similar to get_value, but mutable
fn set_value(doc: &mut DocumentMut, selector: &str, new_value: &str) -> Result<()> {
    let keys = split_selector(selector)?;
    if keys.len() == 1 {
        let key = keys[0];
        if doc.get(key).is_none() {
            return Err(anyhow::anyhow!("Key not found: {}", key));
        }
        doc[key] = value(new_value);
        return Ok(());
    }
    let mut processed_keys = Vec::new();
    let mut item = doc
        .get_mut(keys[0])
        .ok_or_else(|| anyhow::anyhow!("Key not found: {}", keys[0]))?;
    processed_keys.push(keys[0]);
    assert_not_array_of_tables(item)?;
    for key in &keys[1..keys.len() - 1] {
        let table = item.as_table_like_mut().ok_or_else(|| {
            anyhow::anyhow!("Value for '{}' is not a table", processed_keys.join("."))
        })?;
        let next_path = format!("{}.{}", processed_keys.join("."), key);
        item = table
            .get_mut(key)
            .ok_or_else(|| anyhow::anyhow!("Key '{}' not found", next_path))?;
        processed_keys.push(*key);
        assert_not_array_of_tables(item)?;
    }
    let table = item.as_table_like_mut().ok_or_else(|| {
        anyhow::anyhow!("Value for '{}' is not a table", processed_keys.join("."))
    })?;
    let last_key = keys[keys.len() - 1];
    let last_path = format!("{}.{}", processed_keys.join("."), last_key);
    let target = table
        .get_mut(last_key)
        .ok_or_else(|| anyhow::anyhow!("Key '{}' not found", last_path))?;
    *target = value(new_value);
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

    #[test]
    fn test_query_invalid_selector_empty() {
        let file_content = r#"version = "1.0.0""#;
        let result = query(file_content, "");
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("Selector cannot be empty"));
    }

    #[test]
    fn test_edit_invalid_selector_dots() {
        let file_content = r#"version = "1.0.0""#;
        let result = edit(file_content, "version.", "1.0.1");
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("Invalid selector"));
    }
}
