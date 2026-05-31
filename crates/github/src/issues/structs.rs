use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchIssuesResponse {
    pub items: Vec<Issue>,
}

#[derive(Deserialize)]
pub struct Issue {
    pub title: String,
    pub number: u64,
    pub pull_request: PullRequest,
}

#[derive(Deserialize)]
pub struct PullRequest {
    pub html_url: String,
}
