mod error;
mod swelog_config;

use std::{
    fs,
    path::PathBuf,
};

pub use error::ConfigAlreadyExists;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};
use swelog_config::SwelogConfig;

const APP_NAME: &str = "swelog";
const CONFIG_FILE_NAME: &str = "swelog.json";

pub fn initialize_config_file(overwrite_existing_config: bool) -> Result<PathBuf> {
    let home_directory = get_home_directory()?;

    let default_config = SwelogConfig::get_default_config(&home_directory);

    let config_file_path = get_config_file_path()?;

    write_default_config(&config_file_path, &default_config, overwrite_existing_config)?;

    Ok(config_file_path)
}

fn get_home_directory() -> Result<PathBuf> {
    dirs::home_dir().ok_or_else(|| miette!("unable to determine home directory"))
}

fn get_config_file_path() -> Result<PathBuf> {
    let config_directory =
        dirs::config_dir().ok_or_else(|| miette!("unable to determine config directory"))?;

    let config_file_path = config_directory.join(APP_NAME).join(CONFIG_FILE_NAME);

    Ok(config_file_path)
}

fn write_default_config(
    config_file_path: &PathBuf,
    config: &SwelogConfig,
    overwrite_existing_config: bool,
) -> Result<()> {
    if config_file_path.exists() && !overwrite_existing_config {
        let config_already_exists_error = ConfigAlreadyExists { path: config_file_path.clone() };

        return Err(config_already_exists_error.into());
    }

    // create the directory that will contain the config file
    if let Some(parent) = config_file_path.parent() {
        fs::create_dir_all(parent).into_diagnostic().wrap_err_with(|| {
            format!("failed to create config directory at {}", parent.display())
        })?;
    }

    let json = serde_json::to_string_pretty(config)
        .into_diagnostic()
        .wrap_err("failed to serialize config")?;

    fs::write(config_file_path, format!("{json}\n")).into_diagnostic().wrap_err_with(|| {
        format!("failed to write config file at {}", config_file_path.display())
    })?;

    Ok(())
}

#[cfg(test)]
mod tests;
