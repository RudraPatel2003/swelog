use chrono::{
    Local,
    NaiveDate,
};
use clap::Args;
use config::utils::read_config_file;
use github::{
    github_token::get_github_token,
    issues::{
        get_merged_prs,
        get_opened_prs,
    },
    users::get_github_username,
    utils::get_repository_name_from_repository_url,
};
use logging::work_file::upsert_managed_work_file_section_from_config;
use miette::Result;

#[derive(Debug, Args)]
pub struct GithubArgs {
    /// Date to fetch GitHub activity for in the format MM-DD-YYYY.
    #[arg(long, value_name = "MM-DD-YYYY", value_parser = parse_activity_date)]
    date: Option<NaiveDate>,
}

impl GithubArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let github_token = get_github_token()?;

        let github_username = get_github_username(&github_token).await?;

        let activity_date = self.date.unwrap_or_else(|| Local::now().date_naive());

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

        upsert_managed_work_file_section_from_config(
            &swelog_config,
            "github",
            "GitHub",
            &github_activity,
        )?;

        Ok(())
    }
}

fn parse_activity_date(date: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date, "%m-%d-%Y")
        .map_err(|_| format!("invalid date `{date}`; expected MM-DD-YYYY"))
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

#[cfg(test)]
mod tests;
