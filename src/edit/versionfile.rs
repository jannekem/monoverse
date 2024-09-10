use anyhow::Result;

use super::LineContext;

/// Update a plain text version file
pub fn edit(new_version: &str) -> Result<String> {
    Ok(new_version.to_string())
}

/// Query a plain text version file for a value
/// Returns the first line of the file
pub fn query(file_content: &str) -> Result<LineContext> {
    let value = file_content
        .lines()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Failed to get value from plain text file"))?
        .to_string();
    Ok(LineContext {
        value,
        line_number: 1,
    })
}
