mod errors;
mod formatting;

use chrono::{
    Local,
    NaiveDate,
};
use clap::Args;
use config::utils::read_config_file;
use dates::{
    date_format::DATE_VALUE_NAME,
    formatting::format_date,
    parsing::parse_date,
};
use linear::client::{
    get_assigned_issues_on_date,
    get_current_active_assigned_issues,
};
use logging::work_file::{
    remove_work_file_section_from_config,
    upsert_work_file_section_from_config,
};
use miette::Result;

use crate::commands::{
    date_selection::{
        DateSelection,
        resolve_selected_date,
    },
    fetch::linear::{
        errors::MissingLinearUsername,
        formatting::format_linear_issues,
    },
};

const LINEAR_SECTION_TITLE: &str = "Linear";

#[derive(Debug, Args)]
pub struct LinearArgs {
    /// Date to fetch Linear activity for in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Fetch the Linear issues you worked on yesterday.
    #[arg(long = "yesterday", conflicts_with = "date")]
    use_yesterday: bool,
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

        let date_selection = DateSelection::from_date_flags(self.date, self.use_yesterday);

        let today = Local::now().date_naive();

        let activity_date = resolve_selected_date(date_selection, today)?;

        let issues = match activity_date {
            Some(activity_date) => {
                get_assigned_issues_on_date(linear_username, &activity_date).await?
            }

            None => get_current_active_assigned_issues(linear_username, &today).await?,
        };

        if issues.is_empty() {
            remove_work_file_section_from_config(&swelog_config, LINEAR_SECTION_TITLE)?;

            println!("{}", format_empty_message(activity_date.as_ref()));

            return Ok(());
        }

        upsert_work_file_section_from_config(
            &swelog_config,
            LINEAR_SECTION_TITLE,
            &format_linear_issues(&issues),
        )?;

        println!("{}", format_recorded_message(issues.len(), activity_date.as_ref()));

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
