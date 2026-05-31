mod structs;

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    header::ACCEPT,
};
use structs::{
    Issue,
    SearchIssuesResponse,
};

use crate::{
    errors::FailedToSendGitHubRequest,
    utils::GITHUB_ACCEPT_HEADER,
};

const MERGED_PRS_API_URL: &str = "https://api.github.com/search/issues";

pub async fn get_merged_prs(github_token: &str) -> Result<Vec<Issue>> {
    let client = Client::new();

    let response = client
        .get(MERGED_PRS_API_URL)
        .query(&[("q", "is:pr is:merged author:@me")])
        .bearer_auth(github_token)
        .header(ACCEPT, GITHUB_ACCEPT_HEADER)
        .send()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToSendGitHubRequest)?;

    let response_text =
        response.text().await.into_diagnostic().wrap_err("failed to read GitHub response body")?;

    let issues = parse_search_issues_response_text(&response_text)?;

    Ok(issues)
}

fn parse_search_issues_response_text(response_text: &str) -> Result<Vec<Issue>> {
    let search_issues_response: SearchIssuesResponse = serde_json::from_str(response_text)
        .into_diagnostic()
        .wrap_err("failed to parse GitHub search issues response")?;

    Ok(search_issues_response.items)
}

#[cfg(test)]
mod tests;
