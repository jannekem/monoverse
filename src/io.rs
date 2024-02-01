use anyhow::{Context, Result};
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P, repo_path: P) -> Result<String> {
    let path = repo_path.as_ref().join(path.as_ref());
    let version_file_content = std::fs::read_to_string(&path)
        .with_context(|| format!("Could not read file at: {:}", path.display()))?;
    Ok(version_file_content)
}

pub fn write_file<P: AsRef<Path>>(path: P, repo_path: P, content: &str) -> Result<()> {
    let path = repo_path.as_ref().join(path.as_ref());
    std::fs::write(&path, content)?;
    Ok(())
}
