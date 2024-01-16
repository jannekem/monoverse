use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod git;
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

    let repo = Repository::open(&opts.repo_path.unwrap())?;
    let has_changed = git::has_path_changed(&repo, &opts.app)?;
    println!("Has changed: {:?}", has_changed);
    Ok(())
}
