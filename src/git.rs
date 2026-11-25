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

/// Commit changes to a list of files, skipping ignored files unless already tracked.
pub fn commit_files<P: AsRef<Path>>(repo: &Repository, files: &[P], message: &str) -> Result<Oid> {
    let mut index = repo.index()?;
    for file in files {
        let path = file.as_ref();
        if index.get_path(path, 0).is_none() && repo.is_path_ignored(path)? {
            continue;
        }
        index.add_path(path)?;
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
    repo.tag(tag, commit.as_object(), &repo.signature()?, tag, false)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf, time::SystemTime};

    const CHART: &str = "charts/app/Chart.yaml";
    const LOCK: &str = "charts/app/Chart.lock";
    const ARCHIVE: &str = "charts/app/charts/common-0.1.0.tgz";

    struct TestRepo {
        repo: Repository,
        path: PathBuf,
    }

    impl TestRepo {
        fn new() -> Self {
            let nonce = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir()
                .join(format!("monoverse-git-test-{}-{nonce}", std::process::id()));
            fs::create_dir(&path).unwrap();
            let repo = Repository::init(&path).unwrap();
            let mut config = repo.config().unwrap();
            config.set_str("user.name", "Monoverse Test").unwrap();
            config
                .set_str("user.email", "monoverse@example.com")
                .unwrap();
            fs::create_dir_all(path.join("charts/app/charts")).unwrap();
            fs::write(path.join(".gitignore"), "charts/*/charts/*.tgz\n").unwrap();
            fs::write(path.join(CHART), "version: 0.1.0\n").unwrap();
            fs::write(path.join(LOCK), "digest: old\n").unwrap();
            let mut index = repo.index().unwrap();
            for file in [".gitignore", CHART, LOCK] {
                index.add_path(Path::new(file)).unwrap();
            }
            index.write().unwrap();
            let tree_id = index.write_tree().unwrap();
            {
                let tree = repo.find_tree(tree_id).unwrap();
                let signature = repo.signature().unwrap();
                repo.commit(Some("HEAD"), &signature, &signature, "initial", &tree, &[])
                    .unwrap();
            }
            Self { repo, path }
        }

        fn release(&self) -> git2::Tree<'_> {
            fs::write(self.path.join(CHART), "version: 0.1.1\n").unwrap();
            fs::write(self.path.join(LOCK), "digest: updated\n").unwrap();
            let oid = commit_files(&self.repo, &[CHART, LOCK, ARCHIVE], "release").unwrap();
            let tree = self.repo.find_commit(oid).unwrap().tree().unwrap();
            for (file, expected) in [(CHART, "version: 0.1.1\n"), (LOCK, "digest: updated\n")] {
                let entry = tree.get_path(Path::new(file)).unwrap();
                let blob = self.repo.find_blob(entry.id()).unwrap();
                assert_eq!(blob.content(), expected.as_bytes());
            }
            tree
        }
    }

    impl Drop for TestRepo {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.path).unwrap();
        }
    }

    #[test]
    fn commit_skips_ignored_untracked_helm_archives_but_updates_manifests_and_lockfiles() {
        let fixture = TestRepo::new();
        fs::write(fixture.path.join(ARCHIVE), "generated archive").unwrap();

        let tree = fixture.release();

        assert!(tree.get_path(Path::new(ARCHIVE)).is_err());
        assert!(fixture
            .repo
            .index()
            .unwrap()
            .get_path(Path::new(ARCHIVE), 0)
            .is_none());
        assert_eq!(
            fixture.repo.status_file(Path::new(ARCHIVE)).unwrap(),
            git2::Status::IGNORED
        );
        assert!(fixture.path.join(ARCHIVE).is_file());
    }

    #[test]
    fn commit_updates_tracked_helm_archives_matching_ignore_rules() {
        let fixture = TestRepo::new();
        fs::write(fixture.path.join(ARCHIVE), "original archive").unwrap();
        // Explicitly track the archive, as if it had previously been force-added.
        let mut index = fixture.repo.index().unwrap();
        index.add_path(Path::new(ARCHIVE)).unwrap();
        index.write().unwrap();
        commit_files(&fixture.repo, &[ARCHIVE], "track archive").unwrap();
        fs::write(fixture.path.join(ARCHIVE), "updated archive").unwrap();

        let tree = fixture.release();

        let entry = tree.get_path(Path::new(ARCHIVE)).unwrap();
        let blob = fixture.repo.find_blob(entry.id()).unwrap();
        assert_eq!(blob.content(), b"updated archive");
        assert_eq!(
            fixture.repo.status_file(Path::new(ARCHIVE)).unwrap(),
            git2::Status::CURRENT
        );
    }

    #[test]
    fn commit_adds_untracked_helm_archives_when_not_ignored() {
        let fixture = TestRepo::new();
        fs::write(fixture.path.join(".gitignore"), "").unwrap();
        fs::write(fixture.path.join(ARCHIVE), "generated archive").unwrap();

        let tree = fixture.release();

        let entry = tree.get_path(Path::new(ARCHIVE)).unwrap();
        let blob = fixture.repo.find_blob(entry.id()).unwrap();
        assert_eq!(blob.content(), b"generated archive");
    }
}
