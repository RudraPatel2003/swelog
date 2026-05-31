use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchIssuesResponse {
    pub items: Vec<Issue>,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub title: String,
    pub number: u64,
    pub pull_request: PullRequest,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub html_url: String,
}
