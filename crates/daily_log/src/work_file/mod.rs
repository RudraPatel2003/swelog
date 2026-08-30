use std::fs;

use config::{
    setup::{
        default_files::is_default_work_file_content,
        swelog_paths::SwelogPaths,
    },
    swelog_config::SwelogConfig,
    swelog_file_existence::ensure_swelog_file_exists,
    work_file::{
        create_or_reset_work_file,
        hide_comments::set_hide_comments_flag,
    },
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::errors::WorkFileNotUpdated;

/// Whether the work file keeps its contents after the daily log is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeepWorkFile {
    Yes,
    No,
}

impl KeepWorkFile {
    /// Converts the `--keep` flag clap parsed into the choice it stands for.
    #[must_use]
    pub const fn from_keep_flag(keep: bool) -> Self {
        if keep { Self::Yes } else { Self::No }
    }
}

pub fn read_work_file_notes(swelog_paths: &SwelogPaths) -> Result<String> {
    ensure_swelog_file_exists(&swelog_paths.work_file)?;

    let work_file_content =
        fs::read_to_string(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", swelog_paths.work_file.display())
        })?;

    if is_default_work_file_content(&work_file_content) {
        let work_file_not_updated_error = WorkFileNotUpdated;

        return Err(work_file_not_updated_error.into());
    }

    Ok(work_file_content)
}

pub fn finish_work_file(swelog_config: &SwelogConfig, keep_work_file: KeepWorkFile) -> Result<()> {
    if keep_work_file == KeepWorkFile::Yes {
        return Ok(());
    }

    set_hide_comments_flag()?;

    create_or_reset_work_file(swelog_config)
}
