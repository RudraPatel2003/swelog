mod errors;
mod formatting;

use clap::Args;
use config::utils::read_config_file;
use linear::get_active_assigned_issues;
use logging::work_file::{
    remove_managed_work_file_section_from_config,
    upsert_managed_work_file_section_from_config,
};
use miette::Result;

use crate::commands::fetch::linear::{
    errors::MissingLinearUsername,
    formatting::format_linear_issues,
};

const LINEAR_SECTION_ID: &str = "linear";
const LINEAR_SECTION_TITLE: &str = "Linear";

#[derive(Debug, Args)]
pub struct LinearArgs {}

impl LinearArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let linear_username = swelog_config
            .linear_username
            .as_deref()
            .map(str::trim)
            .filter(|linear_username| !linear_username.is_empty())
            .ok_or(MissingLinearUsername)?;

        let issues = get_active_assigned_issues(linear_username).await?;

        if issues.is_empty() {
            remove_managed_work_file_section_from_config(&swelog_config, LINEAR_SECTION_ID)?;

            println!("No active Linear issues found.");

            return Ok(());
        }

        upsert_managed_work_file_section_from_config(
            &swelog_config,
            LINEAR_SECTION_ID,
            LINEAR_SECTION_TITLE,
            &format_linear_issues(&issues),
        )?;

        println!("Added {} active Linear issues to your work file.", issues.len());

        Ok(())
    }
}
