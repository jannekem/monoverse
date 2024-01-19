use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
mod project_types;
mod settings;
mod status;

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
            let app = settings
                .projects
                .get(&release.app)
                .ok_or_else(|| anyhow::anyhow!("No project found with name: {}", release.app))?;
            log::info!("App: {:?}", app);
            let status = git::has_path_changed(&repo, &app.path)?;
            log::info!("Status: {:?}", status);
        }
    }
    Ok(())
}
