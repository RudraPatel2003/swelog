use clap::Args;
use config::{
    config_file::read_config_file,
    setup::swelog_paths::SwelogPaths,
    work_file::create_or_reset_work_file,
};
use miette::Result;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct ResetArgs {}

impl ResetArgs {
    pub fn run(self) -> Result<()> {
        let _ = self;

        let swelog_config = read_config_file()?;

        let swelog_paths = SwelogPaths::new(&swelog_config);

        create_or_reset_work_file(&swelog_config)?;

        println!("Reset work file at {}", swelog_paths.work_file.display().cyan());

        Ok(())
    }
}
