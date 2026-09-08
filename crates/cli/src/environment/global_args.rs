use std::path::PathBuf;

use clap::Args;

/// Flags accepted by every command.
#[derive(Debug, Args)]
pub struct GlobalArgs {
    /// Path to the swelog config file. Defaults to swelog/swelog.json in your operating
    /// system's configuration directory.
    #[arg(long = "config", global = true, env = "SWELOG_CONFIG_FILE", value_name = "PATH")]
    pub config_file_path: Option<PathBuf>,
}
