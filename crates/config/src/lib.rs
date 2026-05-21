mod swelog_config;

use anyhow::{
    Context,
    Result,
    bail,
};
use std::{
    fs,
    path::PathBuf,
};
use swelog_config::SwelogConfig;

const APP_NAME: &str = "swelog";
const CONFIG_FILE_NAME: &str = "swelog.json";

pub fn initialize_config_file(overwrite_existing_config: bool) -> Result<()> {
    let home_directory = dirs::home_dir().context("unable to determine home directory")?;

    let default_config = SwelogConfig::get_default_config(&home_directory);

    let config_file_path = get_config_file_path()?;

    write_default_config(
        &config_file_path,
        &default_config,
        overwrite_existing_config,
    )?;

    println!("Created config at {}", config_file_path.display());

    Ok(())
}

fn get_config_file_path() -> Result<PathBuf> {
    let config_directory = dirs::config_dir().context("could not determine config directory")?;

    let config_file_path = config_directory.join(APP_NAME).join(CONFIG_FILE_NAME);

    Ok(config_file_path)
}

fn write_default_config(
    config_file_path: &PathBuf,
    config: &SwelogConfig,
    overwrite_existing_config: bool,
) -> Result<()> {
    if config_file_path.exists() && !overwrite_existing_config {
        bail!(
            "config already exists at {}; use --force to overwrite",
            config_file_path.display()
        );
    }

    // create the directory that will contain the config file
    if let Some(parent) = config_file_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("failed to create config directory at {}", parent.display())
        })?;
    }

    let json = serde_json::to_string_pretty(config).context("failed to serialize config")?;

    fs::write(config_file_path, format!("{json}\n")).with_context(|| {
        format!(
            "failed to write config file at {}",
            config_file_path.display()
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod tests;
