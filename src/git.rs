use std::path::Path;

use anyhow::Result;
use git2::{DiffOptions, Oid, Repository};

/// Check if a path has changed since the given commit
pub fn has_path_changed_since<P: AsRef<Path>>(
    repo: &Repository,
    path: &P,
    commit_id: Oid,
) -> Result<bool> {
    let previous_release = repo.find_commit(commit_id)?;
    let head = repo.head()?.peel_to_commit()?;
    log::debug!("Comparing {:?} to {:?}", previous_release, head);
    let mut opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(
        Some(&previous_release.tree()?),
        Some(&head.tree()?),
        Some(&mut opts),
    )?;
    Ok(diff.deltas().any(|delta| match delta.new_file().path() {
        Some(file_path) => {
            log::info!("Comparing {:?} to {:?}", file_path, path.as_ref());
            file_path.starts_with(path)
        }
        None => false,
    }))
}

/// Get the commit ID for a line in a file
pub fn get_commit_id_for_line<P: AsRef<Path>>(
    repo: &Repository,
    path: &P,
    line: usize,
) -> Result<Oid> {
    let blame = repo.blame_file(path.as_ref(), None)?;
    match blame.get_line(line) {
        Some(hunk) => Ok(hunk.final_commit_id()),
        None => Err(anyhow::anyhow!("No commit found for line: {}", line)),
    }
}

/// Commit changes to a list of files
pub fn commit_files<P: AsRef<Path>>(repo: &Repository, files: &[P], message: &str) -> Result<Oid> {
    let mut index = repo.index()?;
    for file in files {
        index.add_path(file.as_ref())?;
    }
    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let author = repo.signature()?;
    let head = repo.head()?;
    let parent = head.peel_to_commit()?;
    match repo.commit(Some("HEAD"), &author, &author, message, &tree, &[&parent]) {
        Ok(oid) => Ok(oid),
        Err(e) => Err(anyhow::anyhow!("Failed to commit: {}", e)),
    }
}

/// Tag commit
pub fn tag_commit(repo: &Repository, commit_id: Oid, tag: &str) -> Result<()> {
    let commit = repo.find_commit(commit_id)?;
    repo.tag(tag, &commit.as_object(), &repo.signature()?, tag, false)?;
    Ok(())
}
