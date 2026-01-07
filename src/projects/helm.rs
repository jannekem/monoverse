use anyhow::Result;
use std::path::PathBuf;

use crate::{settings::ProjectSettings, version::VersionContext};

pub struct HelmProject {
    base: super::BaseProjectFile,
}

impl HelmProject {
    pub fn new(settings: ProjectSettings, repo_path: PathBuf) -> Self {
        Self {
            base: super::BaseProjectFile {
                settings,
                repo_path,
            },
        }
    }
}

impl super::ProjectFile for HelmProject {
    fn base(&self) -> &super::BaseProjectFile {
        &self.base
    }

    fn version_context(&self, version_file_content: &str) -> Result<VersionContext> {
        let line_context = crate::edit::yaml::query(version_file_content, "version")?;
        Ok(VersionContext::from_line_context(line_context))
    }

    fn update_version(
        &self,
        version_file_content: &str,
        version_context: &VersionContext,
    ) -> Result<String> {
        let next_version = version_context.next_version.to_string();
        let mut new_content =
            crate::edit::yaml::edit(version_file_content, "version", &next_version)?;
        if crate::edit::yaml::query(version_file_content, "appVersion").is_ok() {
            new_content = crate::edit::yaml::edit(&new_content, "appVersion", &next_version)?;
        }
        Ok(new_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::ProjectFile;
    use crate::version::ToVersion;

    #[test]
    fn test_version_context() {
        let project = HelmProject::new(
            ProjectSettings {
                project_type: crate::projects::ProjectType::Helm,
                project_path: PathBuf::new(),
                manifest_path: Some("Chart.yaml".into()),
                tag_prefix: None,
                selector: None,
                dependents: None,
            },
            PathBuf::new(),
        );
        let version_file_content = r#"apiVersion: v2
appVersion: "1.2.3"
description: A Helm chart for Kubernetes
name: test
version: 0.1.0"#;
        let result = project.version_context(version_file_content).unwrap();
        assert_eq!(result.version, "0.1.0".to_version());
        assert_eq!(result.line_number, 5);
    }

    #[test]
    fn test_update_version() {
        let project = HelmProject::new(
            ProjectSettings {
                project_type: crate::projects::ProjectType::Helm,
                project_path: PathBuf::new(),
                manifest_path: Some("Chart.yaml".into()),
                tag_prefix: None,
                selector: None,
                dependents: None,
            },
            PathBuf::new(),
        );
        let version_file_content = r#"apiVersion: v2
description: A Helm chart for Kubernetes
name: test
appVersion: "1.2.3"
version: 0.1.0"#;
        let version_context = VersionContext {
            version: "0.1.0".to_version(),
            next_version: "0.1.1".to_version(),
            line_number: 5,
        };
        let result = project
            .update_version(version_file_content, &version_context)
            .unwrap();
        assert_eq!(
            result,
            r#"apiVersion: v2
description: A Helm chart for Kubernetes
name: test
appVersion: "0.1.1"
version: 0.1.1"#
        );
    }

    #[test]
    fn test_update_version_without_app_version() {
        let project = HelmProject::new(
            ProjectSettings {
                project_type: crate::projects::ProjectType::Helm,
                project_path: PathBuf::new(),
                manifest_path: Some("Chart.yaml".into()),
                tag_prefix: None,
                selector: None,
                dependents: None,
            },
            PathBuf::new(),
        );
        let version_file_content = r#"apiVersion: v2
description: A Helm chart for Kubernetes
name: test
version: 0.1.0"#;
        let version_context = VersionContext {
            version: "0.1.0".to_version(),
            next_version: "0.1.1".to_version(),
            line_number: 4,
        };
        let result = project
            .update_version(version_file_content, &version_context)
            .unwrap();
        assert_eq!(
            result,
            r#"apiVersion: v2
description: A Helm chart for Kubernetes
name: test
version: 0.1.1"#
        );
    }
}
