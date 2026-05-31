use clap::Args;
use config::utils::read_config_file;
use logging::work_file::log_to_work_file_from_config;
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

        let formatted_work_item = format!("- {}", self.work_item);

        log_to_work_file_from_config(&swelog_config, &formatted_work_item, "Log")?;

        println!("Logged work item into {}", swelog_config.work_file_name.cyan());

        Ok(())
    }
}
