use clap::Args;
use config::utils::read_config_file;
use logging::log_work_item_from_config;
use miette::Result;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct LogArgs {
    /// Work item to add as a Markdown bullet.
    work_item: String,
}

impl LogArgs {
    pub fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        log_work_item_from_config(&swelog_config, &self.work_item)?;

        println!("Logged work item into {}", swelog_config.work_file_name.cyan());

        Ok(())
    }
}
