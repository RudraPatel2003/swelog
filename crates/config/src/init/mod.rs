use std::{
    fs,
    path::PathBuf,
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::{
    errors::ConfigAlreadyExists,
    swelog_config::SwelogConfig,
};

pub fn write_default_config(
    config_file_path: &PathBuf,
    config: &SwelogConfig,
    overwrite_existing_config: bool,
) -> Result<()> {
    if config_file_path.exists() && !overwrite_existing_config {
        let config_already_exists_error =
            ConfigAlreadyExists { config_file_path: config_file_path.clone() };

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
