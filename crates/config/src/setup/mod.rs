pub mod default_files;
pub mod swelog_paths;

use std::fs;

use default_files::{
    DEFAULT_CONTEXT_FILE_CONTENT,
    DEFAULT_WORK_FILE_CONTENT,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use swelog_paths::SwelogPaths;

use crate::{
    errors::SwelogFilesAlreadyExist,
    swelog_config::SwelogConfig,
};

pub fn setup_swelog_files_from_config(
    swelog_config: &SwelogConfig,
    overwrite_existing_files: bool,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    if !overwrite_existing_files {
        fail_if_swelog_paths_exist(&swelog_paths)?;
    }

    fs::create_dir_all(&swelog_paths.swelog_directory).into_diagnostic().wrap_err_with(|| {
        format!("failed to create swelog directory at {}", swelog_paths.swelog_directory.display())
    })?;

    fs::write(&swelog_paths.context_file, DEFAULT_CONTEXT_FILE_CONTENT)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!("failed to write context file at {}", swelog_paths.context_file.display())
        })?;

    fs::write(&swelog_paths.work_file, DEFAULT_WORK_FILE_CONTENT).into_diagnostic().wrap_err_with(
        || format!("failed to write work file at {}", swelog_paths.work_file.display()),
    )?;

    fs::create_dir_all(&swelog_paths.daily_log_directory).into_diagnostic().wrap_err_with(
        || {
            format!(
                "failed to create daily log directory at {}",
                swelog_paths.daily_log_directory.display()
            )
        },
    )?;

    fs::create_dir_all(&swelog_paths.weekly_log_directory).into_diagnostic().wrap_err_with(
        || {
            format!(
                "failed to create weekly log directory at {}",
                swelog_paths.weekly_log_directory.display()
            )
        },
    )?;

    Ok(())
}

fn fail_if_swelog_paths_exist(swelog_paths: &SwelogPaths) -> Result<()> {
    for swelog_path in swelog_paths.all_paths() {
        if swelog_path.exists() {
            let swelog_files_already_exist_error =
                SwelogFilesAlreadyExist { swelog_path: swelog_path.clone() };

            return Err(swelog_files_already_exist_error.into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
