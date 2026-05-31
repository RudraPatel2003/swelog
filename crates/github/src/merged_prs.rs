use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    header::{
        ACCEPT,
        AUTHORIZATION,
        HeaderValue,
    },
};

use crate::{
    errors::FailedToSendGitHubRequest,
    structs::{
        Issue,
        SearchIssuesResponse,
    },
};

const MERGED_PRS_API_URL: &str = "https://api.github.com/search/issues";

const GITHUB_ACCEPT_HEADER: &str = "application/vnd.github.v3+json";

fn get_authorization_header(github_token: &str) -> Result<HeaderValue> {
    let mut authorization_header = HeaderValue::from_str(&format!("Bearer {}", github_token))
        .into_diagnostic()
        .wrap_err("failed to prepare GitHub authorization header")?;

    authorization_header.set_sensitive(true);

    Ok(authorization_header)
}

pub async fn get_merged_prs(github_token: &str) -> Result<Vec<Issue>> {
    let client = Client::new();

    let authorization_header = get_authorization_header(github_token)?;

    let response = client
        .get(MERGED_PRS_API_URL)
        .query(&[("q", "is:pr is:merged author:@me")])
        .header(AUTHORIZATION, authorization_header)
        .header(ACCEPT, GITHUB_ACCEPT_HEADER)
        .send()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToSendGitHubRequest)?;

    let body = response.text().await?;
    let issues: Vec<Issue> = serde_json::from_str(&body)?;

    Ok(issues)
}
