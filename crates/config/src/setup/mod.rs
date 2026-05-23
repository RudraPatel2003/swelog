mod default_files;
mod setup_paths;

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
use setup_paths::SetupPaths;

use crate::{
    errors::SetupFilesAlreadyExist,
    swelog_config::SwelogConfig,
    utils::read_config_file,
};

pub fn setup_swelog_files(overwrite_existing_files: bool) -> Result<SwelogConfig> {
    let swelog_config = read_config_file()?;

    setup_swelog_files_from_config(&swelog_config, overwrite_existing_files)?;

    Ok(swelog_config)
}

fn setup_swelog_files_from_config(
    swelog_config: &SwelogConfig,
    overwrite_existing_files: bool,
) -> Result<()> {
    let setup_paths = SetupPaths::new(swelog_config);

    if !overwrite_existing_files {
        fail_if_setup_paths_exist(&setup_paths)?;
    }

    fs::create_dir_all(&setup_paths.swelog_directory).into_diagnostic().wrap_err_with(|| {
        format!("failed to create swelog directory at {}", setup_paths.swelog_directory.display())
    })?;

    fs::write(&setup_paths.context_file, DEFAULT_CONTEXT_FILE_CONTENT)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!("failed to write context file at {}", setup_paths.context_file.display())
        })?;

    fs::write(&setup_paths.work_file, DEFAULT_WORK_FILE_CONTENT).into_diagnostic().wrap_err_with(
        || format!("failed to write work file at {}", setup_paths.work_file.display()),
    )?;

    fs::create_dir_all(&setup_paths.daily_log_directory).into_diagnostic().wrap_err_with(|| {
        format!(
            "failed to create daily log directory at {}",
            setup_paths.daily_log_directory.display()
        )
    })?;

    fs::create_dir_all(&setup_paths.weekly_log_directory).into_diagnostic().wrap_err_with(
        || {
            format!(
                "failed to create weekly log directory at {}",
                setup_paths.weekly_log_directory.display()
            )
        },
    )?;

    Ok(())
}

fn fail_if_setup_paths_exist(setup_paths: &SetupPaths) -> Result<()> {
    for setup_path in setup_paths.paths_to_check() {
        if setup_path.exists() {
            let setup_files_already_exist_error =
                SetupFilesAlreadyExist { setup_path: setup_path.to_path_buf() };

            return Err(setup_files_already_exist_error.into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
