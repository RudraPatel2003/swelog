pub mod global_args;

use std::path::PathBuf;

use config::config_file::get_default_config_file_path;
use miette::Result;

use crate::environment::global_args::GlobalArgs;

pub struct Environment {
    pub config_file_path: PathBuf,
}

pub fn resolve_environment(global_args: GlobalArgs) -> Result<Environment> {
    let config_file_path =
        global_args.config_file_path.map_or_else(get_default_config_file_path, Ok)?;

    Ok(Environment { config_file_path })
}
