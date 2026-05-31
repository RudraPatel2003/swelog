use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("daily log already exists at {daily_log_file}")]
#[diagnostic(
    code(swelog::summary::daily_log_already_exists),
    help("use `swelog summarize day --force` to overwrite the existing daily log file")
)]
pub struct DailyLogAlreadyExists {
    pub daily_log_file: PathBuf,
}

#[derive(Debug, Diagnostic, Error)]
#[error("work file not updated")]
#[diagnostic(
    code(swelog::summary::work_file_not_updated),
    help(
        "Update the work file with your work notes before running `swelog summarize` or `swelog summarize day`"
    )
)]
pub struct WorkFileNotUpdated;
