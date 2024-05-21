use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,

    /// Repository path
    #[clap(long, default_value = ".", global = true)]
    pub repo_path: Option<PathBuf>,

    #[clap(flatten)]
    pub verbosity: Verbosity<WarnLevel>,
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
    /// Force release
    #[clap(long, short)]
    pub force: bool,
    /// Commit changes
    #[clap(long)]
    pub commit: bool,
    /// Create a tag
    #[clap(long, requires = "commit")]
    pub tag: bool,
}

#[derive(Parser)]
pub struct Next {
    /// Project name
    pub project: String,
}
