use anyhow::{Context, Result};
use libyaml_safer::{Document, Node, NodeData, NodePair, Parser, ScalarStyle};

use super::LineContext;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SelectorFilter {
    Index(usize),
    KeyValue { key: String, value: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectorPart {
    key: String,
    filter: Option<SelectorFilter>,
    raw: String,
}

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
    let parts = selector
        .split('.')
        .map(parse_selector_part)
        .collect::<Result<Vec<_>>>()?;
    // Start at the root node
    let mut current_node = document
        .get_node(1)
        .ok_or_else(|| anyhow::anyhow!("YAML document does not contain a root node"))?;
    // Keep track of the keys we've processed so far for error messages
    let mut processed_keys = Vec::new();
    for part in parts {
        processed_keys.push(part.raw.clone());
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
        let pair = find_node_pair_by_key(document, pairs, &part.key).with_context(|| {
            format!("Key '{}' not found or not scalar", processed_keys.join("."))
        })?;
        current_node = document.get_node(pair.value).unwrap();
        // If there's a filter, apply it
        if let Some(filter) = part.filter {
            let NodeData::Sequence { items, .. } = &current_node.data else {
                return Err(anyhow::anyhow!(
                    "Value for '{}' is not a sequence",
                    processed_keys.join(".")
                ));
            };
            let next_node = match filter {
                // Return the item at the given index
                SelectorFilter::Index(index) => {
                    items.get(index).and_then(|item| document.get_node(*item))
                }
                // Find the first item where the given key has the given value
                SelectorFilter::KeyValue { key, value } => items.iter().find_map(|item| {
                    let node = document.get_node(*item)?;
                    if let NodeData::Mapping { pairs, .. } = &node.data {
                        let pair = find_node_pair_by_key(document, pairs, &key)?;
                        let value_node = document.get_node(pair.value)?;
                        matches!(
                            &value_node.data,
                            NodeData::Scalar { value: scalar, .. } if scalar == &value
                        )
                        .then_some(node)
                    } else {
                        None
                    }
                }),
            };
            current_node = next_node.ok_or_else(|| {
                anyhow::anyhow!("Key '{}' not found or not scalar", processed_keys.join("."))
            })?;
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
    pairs: &'a [NodePair],
    key: &str,
) -> Option<&'a NodePair> {
    pairs
        .iter()
        .find(|pair| match &document.get_node(pair.key).unwrap().data {
            NodeData::Scalar { value, .. } => value == key,
            _ => false,
        })
}

fn parse_selector_part(raw: &str) -> Result<SelectorPart> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(anyhow::anyhow!("Selector contains an empty segment"));
    }
    if let Some(start) = raw.find('[') {
        if !raw.ends_with(']') {
            return Err(anyhow::anyhow!("Selector segment '{}' is missing ']'", raw));
        }
        let key = raw[..start].trim();
        if key.is_empty() {
            return Err(anyhow::anyhow!(
                "Selector segment '{}' is missing a key",
                raw
            ));
        }
        let inner = &raw[start + 1..raw.len() - 1];
        let filter = match inner.split_once('=') {
            Some((filter_key, filter_value)) => {
                let filter_key = filter_key.trim();
                let filter_value = unquote(filter_value.trim());
                anyhow::ensure!(
                    !filter_value.is_empty(),
                    "Selector segment '{}' has an empty filter value",
                    raw
                );
                anyhow::ensure!(
                    !filter_key.is_empty(),
                    "Selector segment '{}' has an empty filter key",
                    raw
                );
                SelectorFilter::KeyValue {
                    key: filter_key.to_string(),
                    value: filter_value,
                }
            }
            None => {
                let index: usize = inner.trim().parse().map_err(|_| {
                    anyhow::anyhow!("Selector segment '{}' has an invalid index", raw)
                })?;
                SelectorFilter::Index(index)
            }
        };
        Ok(SelectorPart {
            key: key.to_string(),
            filter: Some(filter),
            raw: raw.to_string(),
        })
    } else {
        Ok(SelectorPart {
            key: raw.to_string(),
            filter: None,
            raw: raw.to_string(),
        })
    }
}

fn unquote(value: &str) -> String {
    let mut unquoted = value.to_string();
    if (unquoted.starts_with('"') && unquoted.ends_with('"'))
        || (unquoted.starts_with('\'') && unquoted.ends_with('\''))
    {
        unquoted = unquoted[1..unquoted.len() - 1].to_string();
    }
    unquoted
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
    fn test_query_sequence_index() {
        let file_content = r#"dependencies:
  - name: common
    version: "1.0.0"
  - name: other
    version: "2.0.0""#;
        let selector = "dependencies[0].version";
        let result = query(file_content, selector).unwrap();
        assert_eq!(result.value, "1.0.0");
        assert_eq!(result.line_number, 3);
    }

    #[test]
    fn test_query_sequence_index_scalar_item() {
        let file_content = r#"dependencies:
  - "alpha"
  - "beta""#;
        let selector = "dependencies[0]";
        let result = query(file_content, selector).unwrap();
        assert_eq!(result.value, "alpha");
        assert_eq!(result.line_number, 2);
    }

    #[test]
    fn test_query_sequence_index_out_of_range() {
        let file_content = r#"dependencies:
  - name: common
    version: "1.0.0""#;
        let selector = "dependencies[2].version";
        let result = query(file_content, selector);
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert!(e.to_string().contains("Key 'dependencies[2]' not found")),
        }
    }

    #[test]
    fn test_query_filter_on_non_sequence() {
        let file_content = r#"dependencies:
  name: common
  version: "1.0.0""#;
        let selector = "dependencies[name=common].version";
        let result = query(file_content, selector);
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert!(e
                .to_string()
                .contains("Value for 'dependencies[name=common]' is not a sequence")),
        }
    }

    #[test]
    fn test_query_filter_key_missing_in_item() {
        let file_content = r#"dependencies:
  - version: "1.0.0""#;
        let selector = "dependencies[name=common].version";
        let result = query(file_content, selector);
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert!(e
                .to_string()
                .contains("Key 'dependencies[name=common]' not found")),
        }
    }

    #[test]
    fn test_query_invalid_index_format() {
        let file_content = r#"dependencies:
  - name: common
    version: "1.0.0""#;
        let selector = "dependencies[abc].version";
        let result = query(file_content, selector);
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert!(e
                .to_string()
                .contains("Selector segment 'dependencies[abc]' has an invalid index")),
        }
    }

    #[test]
    fn test_query_missing_closing_bracket() {
        let file_content = r#"dependencies:
  - name: common
    version: "1.0.0""#;
        let selector = "dependencies[0.version";
        let result = query(file_content, selector);
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert!(e
                .to_string()
                .contains("Selector segment 'dependencies[0' is missing ']'")),
        }
    }

    #[test]
    fn test_query_sequence_filter() {
        let file_content = r#"dependencies:
  - name: common
    version: "1.0.0"
  - name: other
    version: "2.0.0""#;
        let selector = "dependencies[name=common].version";
        let result = query(file_content, selector).unwrap();
        assert_eq!(result.value, "1.0.0");
        assert_eq!(result.line_number, 3);
    }

    #[test]
    fn test_edit_sequence_filter() {
        let file_content = r#"dependencies:
  - name: common
    version: "1.0.0"
  - name: other
    version: "2.0.0""#;
        let selector = "dependencies[name=other].version";
        let result = edit(file_content, selector, "2.1.0").unwrap();
        assert_eq!(
            result,
            r#"dependencies:
  - name: common
    version: "1.0.0"
  - name: other
    version: "2.1.0""#
        );
    }

    #[test]
    fn test_query_nested_sequence_filter() {
        let file_content = r#"groups:
  - name: app
    dependencies:
      - name: common
        version: "1.0.0"
      - name: other
        version: "2.0.0""#;
        let selector = "groups[name=app].dependencies[name=common].version";
        let result = query(file_content, selector).unwrap();
        assert_eq!(result.value, "1.0.0");
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
