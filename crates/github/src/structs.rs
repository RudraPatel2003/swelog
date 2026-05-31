pub struct UserResponse {
    pub login: String,
}

pub struct SearchIssuesResponse {
    pub items: Vec<Issue>,
}

pub struct Issue {
    pub title: String,
    pub number: u64,
    pub pull_request: PullRequest,
}

pub struct PullRequest {
    pub merged_at: Option<String>,
}
