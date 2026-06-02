use std::path::PathBuf;

use chrono::NaiveDate;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("weekly log already exists at {weekly_log_file}")]
#[diagnostic(
    code(swelog::summary::weekly_log_already_exists),
    help("use `swelog summarize week --force` to overwrite the existing weekly log file")
)]
pub struct WeeklyLogAlreadyExists {
    pub weekly_log_file: PathBuf,
}

#[derive(Debug, Diagnostic, Error)]
#[error("work file contains unsummarized work")]
#[diagnostic(
    code(swelog::summary::work_file_not_default),
    help(
        "run `swelog summarize` to summarize the current work file, or `swelog reset` to discard it"
    )
)]
pub struct WorkFileNotDefault;

#[derive(Debug, Diagnostic, Error)]
#[error("no daily logs found for week of {monday_date}")]
#[diagnostic(
    code(swelog::summary::no_daily_logs_found),
    help(
        "run `swelog summarize day` for at least one weekday before running `swelog summarize week`"
    )
)]
pub struct NoDailyLogsFound {
    pub monday_date: NaiveDate,
}
