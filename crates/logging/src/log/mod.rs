mod errors;

use std::fs;

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
        read_config_file,
    },
};
use errors::DailyLogAlreadyExists;
use llm::{
    language_model::LanguageModel,
    language_model_factory::get_language_model_from_config,
    prompts::get_daily_log_prompt,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

pub async fn log_daily_work(
    overwrite_existing_daily_log: bool,
    keep_work_file: bool,
) -> Result<NaiveDate> {
    let swelog_config = read_config_file()?;

    let language_model = get_language_model_from_config(&swelog_config);

    let log_date = Local::now().date_naive();

    log_daily_work_from_config(
        &swelog_config,
        language_model.as_ref(),
        log_date,
        overwrite_existing_daily_log,
        keep_work_file,
    )
    .await?;

    Ok(log_date)
}

async fn log_daily_work_from_config(
    swelog_config: &SwelogConfig,
    language_model: &dyn LanguageModel,
    log_date: NaiveDate,
    overwrite_existing_daily_log: bool,
    keep_work_file: bool,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_file_exists(&swelog_paths.context_file)?;
    ensure_swelog_file_exists(&swelog_paths.work_file)?;
    ensure_swelog_directory_exists(&swelog_paths.daily_log_directory)?;

    let daily_log_file_name = get_daily_log_file_name(&log_date);

    let daily_log_file = swelog_paths.daily_log_directory.join(daily_log_file_name);

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

    let prompt = get_daily_log_prompt(&work_file_content, &context_file_content, &log_date);

    let daily_log_content = language_model.generate_response(&prompt).await?;

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

pub fn get_daily_log_file_name(log_date: &NaiveDate) -> String {
    let formatted_date = log_date.format("%m-%d-%Y").to_string();

    format!("{formatted_date}.md")
}

#[cfg(test)]
mod tests;
