use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
mod project_types;
mod settings;
mod version;

use cli::Opts;
use git2::Repository;

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
            let project_settings = settings.project_settings(&release.project)?;
            let project_file =
                project_types::get_project_file(project_settings.clone(), opts.repo_path.unwrap());
            project_file.release(&repo)?;
        }
        cli::SubCommand::Next(next) => {
            let project_settings = settings.project_settings(&next.project)?;
            let project_file =
                project_types::get_project_file(project_settings.clone(), opts.repo_path.unwrap());
            project_file.print_next_version()?;
        }
    }
    Ok(())
}
