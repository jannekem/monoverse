use std::path::Path;

use git2::{DiffOptions, Repository};

use crate::status::ReleaseStatus;

pub fn has_path_changed<P: AsRef<Path>>(
    repo: &Repository,
    path: &P,
) -> Result<ReleaseStatus, git2::Error> {
    let head = repo.head()?.peel_to_commit()?;
    // Get the latest tag or return ReleaseStatus::Uninitialized
    let latest = match repo.find_reference("refs/tags/latest") {
        Ok(reference) => reference.peel_to_commit()?,
        Err(_) => return Ok(ReleaseStatus::Uninitialized),
    };
    let mut opts = DiffOptions::new();
    let diff =
        repo.diff_tree_to_tree(Some(&latest.tree()?), Some(&head.tree()?), Some(&mut opts))?;
    match diff.deltas().any(|delta| match delta.new_file().path() {
        Some(file_path) => file_path.starts_with(path),
        None => false,
    }) {
        true => Ok(ReleaseStatus::Changed),
        false => Ok(ReleaseStatus::Unchanged),
    }
}
