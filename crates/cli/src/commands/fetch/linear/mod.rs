mod errors;
mod formatting;

use chrono::NaiveDate;
use clap::Args;
use config::utils::read_config_file;
use dates::{
    DATE_VALUE_NAME,
    formatting::format_date,
    parsing::parse_date,
};
use linear::{
    get_active_assigned_issues,
    get_assigned_issues_worked_on,
};
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
pub struct LinearArgs {
    /// Date to fetch Linear activity for in the format MM-DD-YYYY. Without this, your currently
    /// active issues are fetched instead.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,
}

impl LinearArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let linear_username = swelog_config
            .linear_username
            .as_deref()
            .map(str::trim)
            .filter(|linear_username| !linear_username.is_empty())
            .ok_or(MissingLinearUsername)?;

        let issues = match self.date {
            Some(activity_date) => {
                get_assigned_issues_worked_on(linear_username, &activity_date).await?
            }

            None => get_active_assigned_issues(linear_username).await?,
        };

        if issues.is_empty() {
            remove_managed_work_file_section_from_config(&swelog_config, LINEAR_SECTION_ID)?;

            println!("{}", format_empty_message(self.date.as_ref()));

            return Ok(());
        }

        upsert_managed_work_file_section_from_config(
            &swelog_config,
            LINEAR_SECTION_ID,
            LINEAR_SECTION_TITLE,
            &format_linear_issues(&issues),
        )?;

        println!("{}", format_recorded_message(issues.len(), self.date.as_ref()));

        Ok(())
    }
}

fn format_empty_message(activity_date: Option<&NaiveDate>) -> String {
    activity_date.map_or_else(
        || "No active Linear issues found.".to_string(),
        |activity_date| format!("No Linear activity found for {}.", format_date(activity_date)),
    )
}

fn format_recorded_message(issue_count: usize, activity_date: Option<&NaiveDate>) -> String {
    activity_date.map_or_else(
        || format!("Added {issue_count} active Linear issues to your work file."),
        |activity_date| {
            format!(
                "Added {issue_count} Linear issues from {} to your work file.",
                format_date(activity_date)
            )
        },
    )
}
