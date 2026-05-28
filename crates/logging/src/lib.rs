use std::{
    fs::{
        self,
        OpenOptions,
    },
    io::Write,
};

use config::{
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
    utils::{
        ensure_swelog_file_exists,
        read_config_file,
    },
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

pub fn log_work_item(work_item: &str) -> Result<()> {
    let swelog_config = read_config_file()?;

    log_work_item_from_config(&swelog_config, work_item)
}

fn log_work_item_from_config(swelog_config: &SwelogConfig, work_item: &str) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_file_exists(&swelog_paths.work_file)?;

    let work_file_content =
        fs::read(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", swelog_paths.work_file.display())
        })?;

    let mut appended_content = String::new();

    if work_file_needs_preceding_newline(&work_file_content) {
        appended_content.push('\n');
    }

    appended_content.push_str("- ");
    appended_content.push_str(work_item);
    appended_content.push('\n');

    let mut work_file = OpenOptions::new()
        .append(true)
        .open(&swelog_paths.work_file)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!("failed to open work file at {}", swelog_paths.work_file.display())
        })?;

    work_file.write_all(appended_content.as_bytes()).into_diagnostic().wrap_err_with(|| {
        format!("failed to append work item to {}", swelog_paths.work_file.display())
    })?;

    Ok(())
}

fn work_file_needs_preceding_newline(work_file_content: &[u8]) -> bool {
    !work_file_content.is_empty() && work_file_content.last() != Some(&b'\n')
}

#[cfg(test)]
mod tests;
