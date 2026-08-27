use std::fs;

use chrono::NaiveDate;
use config::{
    overwrite::Overwrite,
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
    utils::ensure_swelog_directory_exists,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::{
    content::build_daily_log_content,
    file::resolve_daily_log_file,
    work_file::{
        KeepWorkFile,
        finish_work_file,
        read_work_file_notes,
    },
};

pub fn write_daily_log_from_config(
    swelog_config: &SwelogConfig,
    log_date: &NaiveDate,
    overwrite: Overwrite,
    keep_work_file: KeepWorkFile,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_directory_exists(&swelog_paths.daily_log_directory)?;

    let daily_log_file = resolve_daily_log_file(&swelog_paths, log_date, overwrite)?;

    let work_file_content = read_work_file_notes(&swelog_paths)?;

    let daily_log_content = build_daily_log_content(&work_file_content, log_date);

    fs::write(&daily_log_file, daily_log_content).into_diagnostic().wrap_err_with(|| {
        format!("failed to write daily log file at {}", daily_log_file.display())
    })?;

    finish_work_file(swelog_config, keep_work_file)
}

#[cfg(test)]
mod tests;
