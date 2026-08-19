mod errors;

use std::collections::HashMap;

use clap::{
    Args,
    Subcommand,
};
use config::utils::{
    get_linear_oauth_credentials_file_path,
    read_config_file,
};
use errors::MissingLinearUsername;
use linear::{
    LinearIssue,
    LinearStatusType,
    clear_linear_authorization,
    get_active_assigned_issues,
};
use logging::work_file::{
    remove_managed_work_file_section_from_config,
    upsert_managed_work_file_section_from_config,
};
use miette::Result;

#[derive(Debug, Args)]
pub struct LinearArgs {
    #[command(subcommand)]
    command: Option<LinearCommands>,
}

#[derive(Debug, Subcommand)]
enum LinearCommands {
    /// Remove locally stored Linear OAuth credentials
    Logout,
}

impl LinearArgs {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Some(LinearCommands::Logout) => logout(),
            None => fetch().await,
        }
    }
}

fn logout() -> Result<()> {
    let credentials_file = get_linear_oauth_credentials_file_path()?;

    clear_linear_authorization(&credentials_file)?;

    println!("Removed locally stored Linear authorization.");
    println!("The next `swelog fetch linear` command will open the Linear authorization flow.");

    Ok(())
}

async fn fetch() -> Result<()> {
    let swelog_config = read_config_file()?;
    let linear_username = swelog_config
        .linear_username
        .as_deref()
        .map(str::trim)
        .filter(|username| !username.is_empty())
        .ok_or(MissingLinearUsername)?;
    let credentials_file = get_linear_oauth_credentials_file_path()?;
    let issues = get_active_assigned_issues(linear_username, &credentials_file).await?;

    if issues.is_empty() {
        remove_managed_work_file_section_from_config(&swelog_config, "linear")?;
        println!("No active Linear issues found.");

        return Ok(());
    }

    let linear_activity = format_linear_issues(&issues);

    upsert_managed_work_file_section_from_config(
        &swelog_config,
        "linear",
        "Linear",
        &linear_activity,
    )?;

    println!("Added {} active Linear issues to your work file.", issues.len());

    Ok(())
}

fn format_linear_issues(issues: &[LinearIssue]) -> String {
    let mut groups = Vec::<IssueGroup>::new();
    let mut group_indexes = HashMap::<String, usize>::new();

    for issue in issues {
        let group_index = *group_indexes.entry(issue.status_name.clone()).or_insert_with(|| {
            let group_index = groups.len();

            groups.push(IssueGroup {
                status_name: issue.status_name.clone(),
                status_type: issue.status_type.clone(),
                issues: Vec::new(),
            });

            group_index
        });

        if let Some(group) = groups.get_mut(group_index) {
            group.issues.push(issue);
        }
    }

    groups.sort_by(|left, right| {
        status_sort_key(&left.status_type)
            .cmp(&status_sort_key(&right.status_type))
            .then_with(|| left.status_name.cmp(&right.status_name))
    });

    groups
        .into_iter()
        .map(|group| {
            let issue_lines =
                group.issues.into_iter().map(format_linear_issue).collect::<Vec<_>>().join("\n");

            format!("### {}\n{issue_lines}", group.status_name)
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn format_linear_issue(issue: &LinearIssue) -> String {
    let identifier = escape_markdown_link_text(&issue.identifier);
    let title = issue.title.split_whitespace().collect::<Vec<_>>().join(" ");
    let title = escape_markdown_link_text(&title);

    format!("- [{identifier}]({}) {title}", issue.url)
}

fn escape_markdown_link_text(text: &str) -> String {
    text.replace('\\', "\\\\").replace('[', "\\[").replace(']', "\\]")
}

const fn status_sort_key(status_type: &LinearStatusType) -> u8 {
    match status_type {
        LinearStatusType::Started => 0,
        LinearStatusType::Unstarted => 1,
        LinearStatusType::Backlog => 2,
        LinearStatusType::Other(_) => 3,
        LinearStatusType::Completed => 4,
        LinearStatusType::Canceled => 5,
    }
}

struct IssueGroup<'a> {
    status_name: String,
    status_type: LinearStatusType,
    issues: Vec<&'a LinearIssue>,
}

#[cfg(test)]
mod tests;
