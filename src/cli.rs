use clap::Parser;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
pub struct Opts {
    /// The application to be updated
    pub app: String,

    /// Repository path
    #[clap(long, default_value = ".")]
    pub repo_path: Option<String>,
}