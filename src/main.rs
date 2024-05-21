use std::path::PathBuf;

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
        .verbosity(opts.verbosity.log_level_filter())
        .init()
        .unwrap();
    let settings = settings::Settings::new(opts.repo_path.as_ref().unwrap())?;
    log::info!("Settings: {:?}", settings);
    match opts.subcmd {
        cli::SubCommand::Release(release) => {
            handle_release(release, settings, opts.repo_path.unwrap())?;
        }
        cli::SubCommand::Next(next) => {
            handle_next(next, settings, opts.repo_path.unwrap())?;
        }
    }
    Ok(())
}

fn handle_release(
    release: cli::Release,
    settings: settings::Settings,
    repo_path: PathBuf,
) -> Result<()> {
    let repo = Repository::open(&repo_path)?;
    let project_settings = settings.project_settings(&release.project)?;
    let project_file = projects::get_project_file(project_settings.clone(), repo_path.clone());
    let dependents = project_settings
        .dependents
        .iter()
        .flatten()
        .map(|dependent| dependents::get_dependent(dependent, repo_path.clone()))
        .collect::<Result<Vec<_>>>()?;
    if let Some(version) = project_file
        .release(&repo, release.force)
        .with_context(|| format!("Failed to release '{}'", release.project))?
    {
        let mut file_paths = Vec::new();
        for dependent in dependents {
            dependent.update_version(&version)?;
            file_paths.push(dependent.get_file_path());
        }
        if release.commit {
            file_paths.push(project_file.get_manifest_file_path()?);
            let commit_id = git::commit_files(
                &repo,
                &file_paths,
                &format!("chore: release {} {}", release.project, version),
            )?;
            if release.tag {
                let tag_prefix = project_settings
                    .tag_prefix
                    .clone()
                    .unwrap_or_else(|| format!("{}-", release.project));
                let tag = format!("{}{}", tag_prefix, version);
                git::tag_commit(&repo, commit_id, &tag)?;
            }
        }
        println!("{}", version);
    }
    Ok(())
}

fn handle_next(next: cli::Next, settings: settings::Settings, repo_path: PathBuf) -> Result<()> {
    let project_settings = settings.project_settings(&next.project)?;
    let project_file = projects::get_project_file(project_settings.clone(), repo_path);
    project_file.print_next_version()?;
    Ok(())
}
