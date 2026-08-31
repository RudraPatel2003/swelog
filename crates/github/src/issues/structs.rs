use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchIssuesResponse {
    pub items: Vec<Issue>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Issue {
    pub title: String,
    pub number: u64,
    pub pull_request: PullRequest,
    pub repository_url: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PullRequest {
    pub html_url: String,
}
