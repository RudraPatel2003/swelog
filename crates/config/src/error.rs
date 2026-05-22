use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("config already exists at {path}")]
#[diagnostic(
    code(swelog::config::config_already_exists),
    help("use --force to overwrite the existing config file")
)]
pub struct ConfigAlreadyExists {
    pub path: PathBuf,
}
