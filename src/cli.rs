use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,

    /// Repository path
    #[clap(long, default_value = ".", global = true)]
    pub repo_path: Option<PathBuf>,
}

#[derive(Parser)]
pub enum SubCommand {
    /// Release a project
    Release(Release),
    /// Print the next version for a project
    Next(Next),
}

#[derive(Parser)]
pub struct Release {
    /// Project name
    pub project: String,
}

#[derive(Parser)]
pub struct Next {
    /// Project name
    pub project: String,
}
