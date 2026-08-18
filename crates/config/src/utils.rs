use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};

use crate::{
    errors::{
        ConfigNotFound,
        EmptyObsidianVaultPath,
        SwelogFileNotFound,
    },
    swelog_config::SwelogConfig,
};

const APP_NAME: &str = "swelog";
const CONFIG_FILE_NAME: &str = "swelog.json";
const LINEAR_OAUTH_CREDENTIALS_FILE_NAME: &str = "linear-oauth.json";

pub fn get_config_file_path() -> Result<PathBuf> {
    let config_directory =
        dirs::config_dir().ok_or_else(|| miette!("unable to determine config directory"))?;

    let config_file_path = config_directory.join(APP_NAME).join(CONFIG_FILE_NAME);

    Ok(config_file_path)
}

pub fn get_linear_oauth_credentials_file_path() -> Result<PathBuf> {
    let config_file_path = get_config_file_path()?;

    Ok(config_file_path.with_file_name(LINEAR_OAUTH_CREDENTIALS_FILE_NAME))
}

pub fn read_config_file() -> Result<SwelogConfig> {
    let config_file_path = get_config_file_path()?;

    if !config_file_path.exists() {
        let config_not_found_error = ConfigNotFound { config_file_path };

        return Err(config_not_found_error.into());
    }

    let config_file_contents =
        fs::read_to_string(&config_file_path).into_diagnostic().wrap_err_with(|| {
            format!("failed to read config file at {}", config_file_path.display())
        })?;

    let config: SwelogConfig = serde_json::from_str(&config_file_contents)
        .into_diagnostic()
        .wrap_err("failed to parse config")?;

    if config.obsidian_vault_path.as_os_str().is_empty() {
        let empty_obsidian_vault_path_error = EmptyObsidianVaultPath { config_file_path };

        return Err(empty_obsidian_vault_path_error.into());
    }

    Ok(config)
}

pub fn ensure_swelog_file_exists(swelog_path: &Path) -> Result<()> {
    if swelog_path.is_file() {
        return Ok(());
    }

    let swelog_file_not_found_error = SwelogFileNotFound { swelog_path: swelog_path.to_path_buf() };

    Err(swelog_file_not_found_error.into())
}

pub fn ensure_swelog_directory_exists(swelog_path: &Path) -> Result<()> {
    if swelog_path.is_dir() {
        return Ok(());
    }

    let swelog_file_not_found_error = SwelogFileNotFound { swelog_path: swelog_path.to_path_buf() };

    Err(swelog_file_not_found_error.into())
}
