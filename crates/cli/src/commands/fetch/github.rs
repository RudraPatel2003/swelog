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
    repository_name::get_repository_name_from_repository_url,
    users::get_github_username,
};
use markdown::work_file::upsert_work_file_section_from_config;
use miette::Result;

use crate::shared::date_selection::{
    DateSelection,
    resolve_selected_date,
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

        let (opened_prs, merged_prs) = tokio::try_join!(
            get_opened_prs(&github_token, &github_username, &activity_date),
            get_merged_prs(&github_token, &github_username, &activity_date),
        )?;

        if opened_prs.is_empty() && merged_prs.is_empty() {
            println!("No GitHub activity found.");

            return Ok(());
        }

        let mut github_activity_lines = Vec::new();

        for opened_pr in opened_prs {
            let github_activity_line = format_pull_request_activity(
                "Opened",
                &opened_pr.title,
                opened_pr.number,
                &opened_pr.pull_request.html_url,
                &opened_pr.repository_url,
            );

            github_activity_lines.push(github_activity_line);
        }

        for merged_pr in merged_prs {
            let github_activity_line = format_pull_request_activity(
                "Merged",
                &merged_pr.title,
                merged_pr.number,
                &merged_pr.pull_request.html_url,
                &merged_pr.repository_url,
            );

            github_activity_lines.push(github_activity_line);
        }

        let github_activity = github_activity_lines.join("\n");

        upsert_work_file_section_from_config(
            &swelog_config,
            GITHUB_SECTION_TITLE,
            &github_activity,
        )?;

        println!("Recorded {} GitHub PRs in your work file.", github_activity_lines.len());

        Ok(())
    }
}

fn format_pull_request_activity(
    action: &str,
    title: &str,
    number: u64,
    pull_request_url: &str,
    repository_url: &str,
) -> String {
    let repository_name = get_repository_name_from_repository_url(repository_url);

    let repository_html_url = format!("https://github.com/{repository_name}");

    format!(
        "- {action} \"{title}\" ([#{number}]({pull_request_url})) in [{repository_name}]({repository_html_url})"
    )
}
