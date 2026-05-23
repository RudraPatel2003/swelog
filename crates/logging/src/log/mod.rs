use std::{
    fs,
    path::PathBuf,
};

use chrono::{
    Local,
    NaiveDate,
};
use config::{
    setup::{
        DEFAULT_WORK_FILE_CONTENT,
        SwelogPaths,
    },
    swelog_config::SwelogConfig,
    utils::{
        ensure_swelog_directory_exists,
        ensure_swelog_file_exists,
    },
};
use llm::{
    llm::Llm,
    llm_provider::get_llm_provider_from_config,
};
use miette::{
    Diagnostic,
    IntoDiagnostic,
    Result,
    WrapErr,
};
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

pub async fn log_daily_work(
    overwrite_existing_daily_log: bool,
    keep_work_file: bool,
) -> Result<()> {
    let swelog_config = config::utils::read_config_file()?;

    let llm = get_llm_provider_from_config(&swelog_config);

    let log_date = Local::now().date_naive();

    log_daily_work_from_config(
        &swelog_config,
        &llm,
        log_date,
        overwrite_existing_daily_log,
        keep_work_file,
    )
    .await?;

    Ok(())
}

async fn log_daily_work_from_config<L: Llm + ?Sized>(
    swelog_config: &SwelogConfig,
    llm: &L,
    log_date: NaiveDate,
    overwrite_existing_daily_log: bool,
    keep_work_file: bool,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_file_exists(&swelog_paths.context_file)?;
    ensure_swelog_file_exists(&swelog_paths.work_file)?;
    ensure_swelog_directory_exists(&swelog_paths.daily_log_directory)?;

    let daily_log_file = swelog_paths.daily_log_directory.join(daily_log_file_name(log_date));

    if daily_log_file.exists() && !overwrite_existing_daily_log {
        let daily_log_already_exists_error = DailyLogAlreadyExists { daily_log_file };

        return Err(daily_log_already_exists_error.into());
    }

    let context_file_content =
        fs::read_to_string(&swelog_paths.context_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read context file at {}", swelog_paths.context_file.display())
        })?;

    let work_file_content =
        fs::read_to_string(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", swelog_paths.work_file.display())
        })?;

    let prompt = llm::prompts::get_daily_log_prompt(&work_file_content, &context_file_content);
    let daily_log_content = llm.generate_response(&prompt).await?;

    fs::write(&daily_log_file, daily_log_content).into_diagnostic().wrap_err_with(|| {
        format!("failed to write daily log file at {}", daily_log_file.display())
    })?;

    if !keep_work_file {
        fs::write(&swelog_paths.work_file, DEFAULT_WORK_FILE_CONTENT)
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("failed to reset work file at {}", swelog_paths.work_file.display())
            })?;
    }

    Ok(())
}

fn daily_log_file_name(log_date: NaiveDate) -> String {
    format!("{}.md", log_date.format("%m-%d-%Y"))
}

#[cfg(test)]
mod tests;
