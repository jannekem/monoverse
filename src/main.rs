use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
mod project_types;
mod settings;
mod version;

use cli::Opts;
use git2::Repository;

use crate::project_types::{ProjectFile, ProjectType};

fn main() {
    if let Err(e) = run() {
        log::error!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let opts = Opts::parse();
    stderrlog::new()
        .module(module_path!())
        .verbosity(log::Level::Info)
        .init()
        .unwrap();
    let settings = settings::Settings::new(opts.repo_path.as_ref().unwrap())?;
    log::info!("Settings: {:?}", settings);
    let repo = Repository::open(opts.repo_path.as_ref().unwrap())?;
    match opts.subcmd {
        cli::SubCommand::Release(release) => {
            let app_settings = settings
                .projects
                .get(&release.app)
                .ok_or_else(|| anyhow::anyhow!("No project found with name: {}", release.app))?;
            log::info!("App: {:?}", app_settings);
            match app_settings.project_type {
                ProjectType::Helm => {
                    let helm_project = project_types::helm::HelmProject::new(
                        app_settings.clone(),
                        opts.repo_path.as_ref().unwrap().clone(),
                    );
                    helm_project.release(&repo)?;
                }
                ProjectType::Node => {
                    let node_project = project_types::node::NodeProject::new(
                        app_settings.clone(),
                        opts.repo_path.as_ref().unwrap().clone(),
                    );
                    node_project.release(&repo)?;
                }
                ProjectType::Rust => {
                    let rust_project = project_types::rust::RustProject::new(
                        app_settings.clone(),
                        opts.repo_path.as_ref().unwrap().clone(),
                    );
                    rust_project.release(&repo)?;
                }
            }
        }
    }
    Ok(())
}
