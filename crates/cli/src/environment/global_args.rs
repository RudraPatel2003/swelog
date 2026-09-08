use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
pub struct GlobalArgs {
    /// Custom path to a swelog config file
    #[arg(long = "config", global = true, env = "SWELOG_CONFIG_FILE", value_name = "PATH")]
    pub config_file_path: Option<PathBuf>,
}
