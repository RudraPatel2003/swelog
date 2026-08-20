use std::fs;

use clap::Args;
use config::{
    setup::{
        default_files::DEFAULT_WORK_FILE_CONTENT,
        swelog_paths::SwelogPaths,
    },
    utils::read_config_file,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct ResetArgs {}

impl ResetArgs {
    pub fn run(self) -> Result<()> {
        let _ = self;

        let swelog_config = read_config_file()?;

        let swelog_paths = SwelogPaths::new(&swelog_config);

        fs::write(&swelog_paths.work_file, DEFAULT_WORK_FILE_CONTENT)
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("failed to write work file at {}", swelog_paths.work_file.display())
            })?;

        println!("Reset work file at {}", swelog_paths.work_file.display().cyan());

        Ok(())
    }
}
