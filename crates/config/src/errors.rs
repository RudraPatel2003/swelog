use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("config already exists at {config_file_path}")]
#[diagnostic(
    code(swelog::config::config_already_exists),
    help("use `swelog init --force` to overwrite the existing config file")
)]
pub struct ConfigAlreadyExists {
    pub config_file_path: PathBuf,
}

#[derive(Debug, Diagnostic, Error)]
#[error("empty Obsidian vault path in config file at {config_file_path}")]
#[diagnostic(
    code(swelog::config::empty_obsidian_vault_path),
    help("set the absolute path to your Obsidian vault in the config file")
)]
pub struct EmptyObsidianVaultPath {
    pub config_file_path: PathBuf,
}

#[derive(Debug, Diagnostic, Error)]
#[error("config not found at {config_file_path}")]
#[diagnostic(
    code(swelog::config::config_not_found),
    help("run `swelog init` to create a config file")
)]
pub struct ConfigNotFound {
    pub config_file_path: PathBuf,
}

#[derive(Debug, Diagnostic, Error)]
#[error("swelog setup files already exist at {swelog_path}")]
#[diagnostic(
    code(swelog::config::swelog_files_already_exist),
    help(
        "use `swelog setup --force` to overwrite existing swelog files. This will overwrite the existing context and work files but keep the contents of the daily and weekly log directories."
    )
)]
pub struct SwelogFilesAlreadyExist {
    pub swelog_path: PathBuf,
}

#[derive(Debug, Diagnostic, Error)]
#[error("swelog setup file not found at {swelog_path}")]
#[diagnostic(
    code(swelog::config::swelog_file_not_found),
    help("run `swelog setup` to create the required swelog files")
)]
pub struct SwelogFileNotFound {
    pub swelog_path: PathBuf,
}
