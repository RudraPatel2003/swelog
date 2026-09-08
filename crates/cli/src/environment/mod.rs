pub mod clients;
pub mod endpoints;
pub mod global_args;
pub mod update_check;

use std::path::PathBuf;

use chrono::{
    Local,
    NaiveDate,
};
use config::{
    cache_directory::get_default_cache_directory,
    config_file::get_default_config_file_path,
};
use miette::Result;

use crate::environment::{
    endpoints::ServiceEndpoints,
    global_args::GlobalArgs,
    update_check::UpdateCheck,
};

pub struct Environment {
    pub config_file_path: PathBuf,
    pub cache_directory: PathBuf,
    pub endpoints: ServiceEndpoints,
    pub update_check: UpdateCheck,
    pub today: NaiveDate,
}

pub fn resolve_environment(global_args: GlobalArgs) -> Result<Environment> {
    let config_file_path =
        global_args.config_file_path.map_or_else(get_default_config_file_path, Ok)?;

    let cache_directory =
        global_args.cache_directory.map_or_else(get_default_cache_directory, Ok)?;

    let today = global_args.today.unwrap_or_else(|| Local::now().date_naive());

    Ok(Environment {
        config_file_path,
        cache_directory,
        endpoints: global_args.endpoints,
        update_check: global_args.update_check,
        today,
    })
}
