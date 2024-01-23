use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
mod project_types;
mod settings;
mod version;

use cli::Opts;
use git2::Repository;

use crate::project_types::ProjectFile;

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
            let app = settings
                .projects
                .get(&release.app)
                .ok_or_else(|| anyhow::anyhow!("No project found with name: {}", release.app))?;
            log::info!("App: {:?}", app);
            // TODO: Hardcoded to package.json for now
            let package_json_path = app.path.join("package.json");
            log::info!("Package.json path: {:?}", package_json_path);
            let release_commit_id = git::get_commit_id_for_line(&repo, &package_json_path, 3)?;
            let has_changed =
                git::has_path_changed_since(&repo, &package_json_path, release_commit_id)?;
            if has_changed {
                match app.project_type {
                    project_types::ProjectType::Node => {
                        let node_project = project_types::node::NodeProject::new(
                            app.path.clone(),
                            opts.repo_path.as_ref().unwrap().clone(),
                        );
                        node_project.bump_version()?;
                    }
                }
            }

            log::info!("Has project changed since last release: {:?}", has_changed);
        }
    }
    Ok(())
}
