use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use chrono::{
    Local,
    NaiveDate,
};
use config::{
    setup::{
        DEFAULT_WORK_FILE_CONTENT,
        SetupPaths,
    },
    swelog_config::SwelogConfig,
};
use miette::{
    Diagnostic,
    IntoDiagnostic,
    Result,
    WrapErr,
};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("swelog setup file not found at {setup_path}")]
#[diagnostic(
    code(swelog::logging::setup_file_not_found),
    help("run `swelog setup` to create the required swelog files")
)]
pub struct SetupFileNotFound {
    pub setup_path: PathBuf,
}

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
    let llm = llm::from_config(&swelog_config);
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

async fn log_daily_work_from_config<L: llm::Llm + ?Sized>(
    swelog_config: &SwelogConfig,
    llm: &L,
    log_date: NaiveDate,
    overwrite_existing_daily_log: bool,
    keep_work_file: bool,
) -> Result<()> {
    let setup_paths = SetupPaths::new(swelog_config);

    ensure_setup_file_exists(&setup_paths.context_file)?;
    ensure_setup_file_exists(&setup_paths.work_file)?;
    ensure_setup_directory_exists(&setup_paths.daily_log_directory)?;

    let daily_log_file = setup_paths.daily_log_directory.join(daily_log_file_name(log_date));

    if daily_log_file.exists() && !overwrite_existing_daily_log {
        let daily_log_already_exists_error = DailyLogAlreadyExists { daily_log_file };

        return Err(daily_log_already_exists_error.into());
    }

    let context_file_content =
        fs::read_to_string(&setup_paths.context_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read context file at {}", setup_paths.context_file.display())
        })?;

    let work_file_content =
        fs::read_to_string(&setup_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", setup_paths.work_file.display())
        })?;

    let prompt = llm::prompts::get_daily_log_prompt(&work_file_content, &context_file_content);
    let daily_log_content = llm.generate_response(&prompt).await?;

    fs::write(&daily_log_file, daily_log_content).into_diagnostic().wrap_err_with(|| {
        format!("failed to write daily log file at {}", daily_log_file.display())
    })?;

    if !keep_work_file {
        fs::write(&setup_paths.work_file, DEFAULT_WORK_FILE_CONTENT)
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("failed to reset work file at {}", setup_paths.work_file.display())
            })?;
    }

    Ok(())
}

fn ensure_setup_file_exists(setup_path: &Path) -> Result<()> {
    if setup_path.is_file() {
        return Ok(());
    }

    let setup_file_not_found_error = SetupFileNotFound { setup_path: setup_path.to_path_buf() };

    Err(setup_file_not_found_error.into())
}

fn ensure_setup_directory_exists(setup_path: &Path) -> Result<()> {
    if setup_path.is_dir() {
        return Ok(());
    }

    let setup_file_not_found_error = SetupFileNotFound { setup_path: setup_path.to_path_buf() };

    Err(setup_file_not_found_error.into())
}

fn daily_log_file_name(log_date: NaiveDate) -> String {
    format!("{}.md", log_date.format("%m-%d-%Y"))
}

#[cfg(test)]
mod tests;
