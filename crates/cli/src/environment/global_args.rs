use std::path::PathBuf;

use clap::Args;

use crate::environment::{
    endpoints::ServiceEndpoints,
    update_check::UpdateCheck,
};

#[derive(Debug, Args)]
pub struct GlobalArgs {
    /// Custom path to a swelog config file
    #[arg(long = "config", global = true, env = "SWELOG_CONFIG_FILE", value_name = "PATH")]
    pub config_file_path: Option<PathBuf>,

    /// Custom path to the swelog cache directory. Used for testing.
    #[arg(long, global = true, env = "SWELOG_CACHE_DIRECTORY", value_name = "PATH", hide = true)]
    pub cache_directory: Option<PathBuf>,

    #[arg(
        long,
        global = true,
        env = "SWELOG_UPDATE_CHECK",
        value_enum,
        default_value_t = UpdateCheck::On,
        hide = true
    )]
    pub update_check: UpdateCheck,

    #[command(flatten)]
    pub endpoints: ServiceEndpoints,
}
