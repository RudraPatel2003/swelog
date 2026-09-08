use github::{
    issues::Issue,
    repository_name::get_repository_name_from_repository_url,
};

pub fn format_github_activity(opened_prs: &[Issue], merged_prs: &[Issue]) -> String {
    let mut sections = Vec::new();

    if !opened_prs.is_empty() {
        sections.push(format_pull_request_section("Opened", opened_prs));
    }

    if !merged_prs.is_empty() {
        sections.push(format_pull_request_section("Merged", merged_prs));
    }

    sections.join("\n\n")
}

fn format_pull_request_section(action: &str, pull_requests: &[Issue]) -> String {
    let pull_request_lines: Vec<String> =
        pull_requests.iter().map(format_pull_request_line).collect();

    format!("### {action}\n{}", pull_request_lines.join("\n"))
}

fn format_pull_request_line(pull_request: &Issue) -> String {
    let repository_name = get_repository_name_from_repository_url(&pull_request.repository_url);

    let repository_link = format!("[{repository_name}](https://github.com/{repository_name})");

    let pull_request_link =
        format!("[#{}]({})", pull_request.number, pull_request.pull_request.html_url);

    format!(r#"- "{}" ({pull_request_link}) in {repository_link}"#, pull_request.title)
}

#[cfg(test)]
mod tests;
