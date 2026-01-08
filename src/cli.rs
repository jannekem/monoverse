use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,

    /// Repository path
    #[arg(long, default_value = ".", global = true)]
    pub repo_path: Option<PathBuf>,

    #[command(flatten)]
    pub verbosity: Verbosity<WarnLevel>,
}

#[derive(Parser)]
pub enum SubCommand {
    /// Release a project
    Release(Release),
    /// Print the next version for a project
    Next(Next),
    /// List projects
    List(List),
}

#[derive(Parser)]
pub struct Release {
    /// Project name
    pub project: String,
    /// Force release
    #[arg(long, short)]
    pub force: bool,
    /// Commit changes
    #[arg(long)]
    pub commit: bool,
    /// Create a tag
    #[arg(long, requires = "commit")]
    pub tag: bool,
    /// Run `helm dependency update` for helm dependents
    #[arg(long)]
    pub helm_dependency_update: bool,
}

#[derive(Parser)]
pub struct Next {
    /// Project name
    pub project: String,
}

#[derive(Parser)]
pub struct List {}
