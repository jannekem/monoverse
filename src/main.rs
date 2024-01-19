use anyhow::Result;
use clap::Parser;

mod cli;
mod git;
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
    let has_changed = git::has_path_changed(&repo, &opts.app)?;
    println!("Has changed: {:?}", has_changed);
    Ok(())
}
