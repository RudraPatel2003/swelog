use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("daily log already exists at {daily_log_file}")]
#[diagnostic(
    code(swelog::logging::daily_log_already_exists),
    help("use `swelog log --force` to overwrite the existing daily log file")
)]
pub struct DailyLogAlreadyExists {
    pub daily_log_file: PathBuf,
}
