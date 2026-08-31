mod formatting;

use chrono::{
    Local,
    NaiveDate,
};
use clap::Args;
use config::utils::read_config_file;
use credentials::{
    credential::Credential,
    resolution::get_or_prompt_for_credential,
};
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_date,
};
use github::{
    issues::{
        get_merged_prs,
        get_opened_prs,
    },
    users::get_github_username,
};
use markdown::work_file::upsert_work_file_section_from_config;
use miette::Result;

use crate::{
    commands::fetch::github::formatting::format_github_activity,
    shared::date_selection::{
        DateSelection,
        resolve_selected_date,
    },
};

const GITHUB_SECTION_TITLE: &str = "GitHub";

#[derive(Debug, Args)]
pub struct GithubArgs {
    /// Date to fetch GitHub activity for in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Fetch GitHub activity for yesterday instead of today.
    #[arg(long = "yesterday", conflicts_with = "date")]
    use_yesterday: bool,
}

impl GithubArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let github_token = get_or_prompt_for_credential(Credential::Github)?;

        let github_username = get_github_username(&github_token).await?;

        let today = Local::now().date_naive();

        let date_selection = DateSelection::from_date_flags(self.date, self.use_yesterday);

        let activity_date = resolve_selected_date(date_selection, today)?.unwrap_or(today);

        println!("Fetching GitHub PRs...");

        let (opened_prs, merged_prs) = tokio::try_join!(
            get_opened_prs(&github_token, &github_username, &activity_date),
            get_merged_prs(&github_token, &github_username, &activity_date),
        )?;

        if opened_prs.is_empty() && merged_prs.is_empty() {
            println!("No GitHub activity found.");

            return Ok(());
        }

        let pull_request_count = opened_prs.len().saturating_add(merged_prs.len());

        upsert_work_file_section_from_config(
            &swelog_config,
            GITHUB_SECTION_TITLE,
            &format_github_activity(&opened_prs, &merged_prs),
        )?;

        println!("Recorded {pull_request_count} GitHub PRs in your work file.");

        Ok(())
    }
}
