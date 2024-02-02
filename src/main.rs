use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod dependents;
mod edit;
mod git;
mod io;
mod projects;
mod settings;
mod version;

use cli::Opts;
use git2::Repository;

fn main() {
    if let Err(e) = run() {
        log::error!("{}", e);
        for cause in e.chain().skip(1) {
            log::error!("{}", cause);
        }
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
            let project_settings = settings.project_settings(&release.project)?;
            let project_file = projects::get_project_file(
                project_settings.clone(),
                opts.repo_path.clone().unwrap(),
            );
            let dependents = project_settings
                .dependents
                .iter()
                .flatten()
                .map(|dependent| {
                    dependents::get_dependent(&dependent, opts.repo_path.clone().unwrap())
                })
                .collect::<Result<Vec<_>>>()?;

            if let Some(version) = project_file
                .release(&repo)
                .with_context(|| format!("Failed to release '{}'", &release.project))?
            {
                for dependent in dependents {
                    dependent.update_version(&version)?;
                }
            }
        }
        cli::SubCommand::Next(next) => {
            let project_settings = settings.project_settings(&next.project)?;
            let project_file =
                projects::get_project_file(project_settings.clone(), opts.repo_path.unwrap());
            project_file.print_next_version()?;
        }
    }
    Ok(())
}
