pub mod global_args;

use std::path::PathBuf;

use config::{
    cache_directory::get_default_cache_directory,
    config_file::get_default_config_file_path,
};
use miette::Result;

use crate::environment::global_args::GlobalArgs;

pub struct Environment {
    pub config_file_path: PathBuf,
    pub cache_directory: PathBuf,
}

pub fn resolve_environment(global_args: GlobalArgs) -> Result<Environment> {
    let config_file_path =
        global_args.config_file_path.map_or_else(get_default_config_file_path, Ok)?;

    let cache_directory =
        global_args.cache_directory.map_or_else(get_default_cache_directory, Ok)?;

    Ok(Environment { config_file_path, cache_directory })
}
