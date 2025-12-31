use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{settings::DependentSettings, version::Version};

use super::Dependent;

#[derive(Debug)]
pub struct HelmDependent {
    pub settings: DependentSettings,
    pub repo_path: PathBuf,
}

impl HelmDependent {
    fn run_helm_dependency_update(&self, chart_dir: &Path) -> Result<()> {
        let output = Command::new("helm")
            .arg("dependency")
            .arg("update")
            .current_dir(chart_dir)
            .output()
            .with_context(|| {
                format!(
                    "Failed to run 'helm dependency update' in {}",
                    chart_dir.display()
                )
            })?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "helm dependency update failed: {}",
                stderr.trim()
            ));
        }
        Ok(())
    }

    fn collect_dependency_artifacts(&self, chart_dir: &Path) -> Result<Vec<PathBuf>> {
        let chart_lock_path = chart_dir.join("Chart.lock");
        let mut file_paths = Vec::new();
        if self.repo_path.join(&chart_lock_path).exists() {
            file_paths.push(chart_lock_path);
        }
        let charts_dir = chart_dir.join("charts");
        let charts_dir_abs = self.repo_path.join(&charts_dir);
        if charts_dir_abs.is_dir() {
            fs::read_dir(&charts_dir_abs)?.try_for_each(|entry| -> Result<()> {
                let path = entry?.path();
                if path.extension().map(|ext| ext == "tgz").unwrap_or(false) {
                    let filename = path
                        .file_name()
                        .ok_or_else(|| anyhow::anyhow!("Chart archive has no filename"))?;
                    file_paths.push(charts_dir.join(filename));
                }
                Ok(())
            })?;
        }
        Ok(file_paths)
    }
}

impl Dependent for HelmDependent {
    fn update_version(
        &self,
        version: &Version,
        options: &super::DependentUpdateOptions,
    ) -> Result<Vec<PathBuf>> {
        let selector = self
            .settings
            .selector
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Selector is required for Helm dependent"))?;
        let chart_dir = &self.settings.dependent_path;
        let chart_yaml_path = chart_dir.join("Chart.yaml");
        let file_content = crate::io::read_file(&chart_yaml_path, &self.repo_path)?;
        let new_file_content =
            crate::edit::yaml::edit(&file_content, &selector, &version.to_string())?;
        crate::io::write_file(&chart_yaml_path, &self.repo_path, new_file_content.as_str())?;
        let mut file_paths = vec![chart_yaml_path.clone()];
        if options.helm_dependency_update {
            self.run_helm_dependency_update(&self.repo_path.join(chart_dir))?;
            file_paths.extend(self.collect_dependency_artifacts(chart_dir)?);
        }
        Ok(file_paths)
    }
}
