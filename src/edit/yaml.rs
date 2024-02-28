use anyhow::Result;
use libyaml_safer::{Document, Node, NodeData, NodePair, Parser, ScalarStyle};

use super::LineContext;

/// Edit a YAML file for a given selector
pub fn edit(file_content: &str, selector: &str, new_value: &str) -> Result<String> {
    log::info!("Editing YAML file for selector: {}", selector);
    let mut document = load_document(file_content)?;
    let value_node = get_value_node(&mut document, selector)?;
    match value_node {
        Node {
            data: NodeData::Scalar { style, .. },
            start_mark,
            end_mark,
            ..
        } => {
            let before = &file_content[..start_mark.index as usize];
            let after = &file_content[end_mark.index as usize..];
            let new_value = match style {
                ScalarStyle::Plain => new_value.to_string(),
                ScalarStyle::SingleQuoted => format!("'{}'", new_value),
                ScalarStyle::DoubleQuoted => format!("\"{}\"", new_value),
                _ => new_value.to_string(),
            };
            Ok(format!("{}{}{}", before, new_value, after))
        }
        _ => Err(anyhow::anyhow!("Value is not a scalar")),
    }
}

/// Query a YAML file for a value at a given selector
pub fn query(file_content: &str, selector: &str) -> Result<LineContext> {
    log::info!("Querying YAML file for selector: {}", selector);
    let mut document = load_document(file_content)?;
    let value_node = get_value_node(&mut document, selector)?;
    match value_node {
        Node {
            data: NodeData::Scalar { value, .. },
            start_mark,
            ..
        } => Ok(LineContext {
            line_number: start_mark.line as usize + 1,
            value: value.to_string(),
        }),
        _ => Err(anyhow::anyhow!("Value is not a scalar")),
    }
}

fn load_document(file_content: &str) -> Result<Document> {
    let mut parser = Parser::new();
    let mut data = file_content.as_bytes();
    parser.set_input_string(&mut data);
    Ok(Document::load(&mut parser)?)
}

/// Get the value node for a given selector
///
/// This function traverses the YAML document, looking for the node that
/// corresponds to the given selector.
fn get_value_node<'a>(document: &'a mut Document, selector: &str) -> Result<&'a Node> {
    // Split the selector into keys
    let keys = selector.split('.').collect::<Vec<_>>();
    // Start at the root node
    let mut current_node = document
        .get_node(1)
        .ok_or_else(|| anyhow::anyhow!("YAML document does not contain a root node"))?;
    // Keep track of the keys we've processed so far for error messages
    let mut processed_keys = Vec::new();
    for key in keys {
        processed_keys.push(key);
        // Get the node pairs for the current mapping
        let pairs = match &current_node.data {
            NodeData::Mapping { pairs, .. } => pairs,
            _ => {
                return Err(anyhow::anyhow!(
                    "Value for '{}' is not a mapping",
                    processed_keys.join(".")
                ))
            }
        };
        // Find the pair that corresponds to the current key
        if let Some(pair) = find_node_pair_by_key(&document, &pairs, key) {
            current_node = document.get_node(pair.value).unwrap();
        } else {
            return Err(anyhow::anyhow!(
                "Key '{}' not found or not scalar",
                processed_keys.join(".")
            ));
        }
    }
    // Return the node for the final key, which should be a scalar
    match current_node.data {
        NodeData::Scalar { .. } => Ok(current_node),
        _ => Err(anyhow::anyhow!(
            "Value for '{}' is not a scalar",
            processed_keys.join(".")
        )),
    }
}

/// Find a node pair by key
///
/// This function searches a list of node pairs for a pair that has a key with
/// the given value. If a matching pair is found, it is returned. Otherwise, None
/// is returned.
fn find_node_pair_by_key<'a>(
    document: &'a Document,
    pairs: &'a Vec<NodePair>,
    key: &str,
) -> Option<&'a NodePair> {
    pairs
        .iter()
        .find(|pair| match &document.get_node(pair.key).unwrap().data {
            NodeData::Scalar { value, .. } => value == key,
            _ => false,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit() {
        let file_content = r#"appVersion: "1.2.3""#;
        let selector = "appVersion";
        let new_value = "1.2.4";
        let result = edit(file_content, selector, new_value).unwrap();
        assert_eq!(result, r#"appVersion: "1.2.4""#);
    }

    #[test]
    fn test_edit_nested() {
        let file_content = r#"single_key: "single_value"
test_key:
    nested_key: "value"
dependencies:
    serde: "1.0""#;
        let selector = "dependencies.serde";
        let new_value = "2.0";
        let result = edit(file_content, selector, new_value).unwrap();
        assert_eq!(
            result,
            r#"single_key: "single_value"
test_key:
    nested_key: "value"
dependencies:
    serde: "2.0""#
        );
    }

    #[test]
    fn test_edit_comment_preservation() {
        let file_content = r#"appVersion: "1.2.3" # comment"#;
        let selector = "appVersion";
        let new_value = "1.2.4";
        let result = edit(file_content, selector, new_value).unwrap();
        assert_eq!(result, r#"appVersion: "1.2.4" # comment"#);
    }

    #[test]
    fn test_query() {
        let file_content = r#"appVersion: "1.2.3""#;
        let selector = "appVersion";
        let result = query(file_content, selector).unwrap();
        assert_eq!(result.value, "1.2.3");
        assert_eq!(result.line_number, 1);
    }

    #[test]
    fn test_query_nested() {
        let file_content = r#"single_key: "single_value"
test_key:
  nested_key: "value"     
dependencies:
  serde: "1.0""#;
        let selector = "dependencies.serde";
        let result = query(file_content, selector).unwrap();
        assert_eq!(result.value, "1.0");
        assert_eq!(result.line_number, 5);
    }

    #[test]
    fn test_query_not_found() {
        let file_content = r#"appVersion: "1.2.3"
                "#;
        let selector = "not_found";
        let result = query(file_content, selector);
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert!(e.to_string().contains("Key 'not_found' not found")),
        }
    }

    #[test]
    fn test_query_nested_not_found() {
        let file_content = r#"single_key: "single_value"
test_key:
    nested_key: "value"
dependencies:
    serde: "1.0""#;
        let selector = "dependencies.not_found";
        let result = query(file_content, selector);
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert!(e
                .to_string()
                .contains("Key 'dependencies.not_found' not found")),
        }
    }

    #[test]
    fn test_query_nested_not_scalar() {
        let file_content = r#"single_key: "single_value"
test_key:
    nested_key: "value"
dependencies:
    serde:
        version: "1.0""#;
        let selector = "dependencies.serde";
        let result = query(file_content, selector);
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert!(e
                .to_string()
                .contains("Value for 'dependencies.serde' is not a scalar")),
        }
    }

    #[test]
    fn test_query_nested_not_mapping() {
        let file_content = r#"single_key: "single_value"
test_key:
    nested_key: "value"
dependencies:
    - serde"#;
        let selector = "dependencies.serde";
        let result = query(file_content, selector);
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert!(e
                .to_string()
                .contains("Value for 'dependencies.serde' is not a mapping")),
        }
    }
}
