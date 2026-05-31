mod structs;

use std::collections::HashMap;

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    header::{
        ACCEPT,
        USER_AGENT,
    },
};
use structs::{
    Issue,
    SearchIssuesResponse,
};

use crate::{
    errors::{
        FailedToSendGitHubRequest,
        UnsuccessfulGitHubResponse,
    },
    utils::{
        GITHUB_ACCEPT_HEADER,
        SWELOG_USER_AGENT,
        get_current_date_in_iso_8601,
    },
};

const MERGED_PRS_API_URL: &str = "https://api.github.com/search/issues";
const OPENED_PRS_API_URL: &str = "https://api.github.com/search/issues";

pub async fn get_merged_prs(github_token: &str, github_username: &str) -> Result<Vec<Issue>> {
    let client = Client::new();

    let current_iso_8601_date = get_current_date_in_iso_8601();

    let search_query =
        format!("author:{github_username} is:pr is:merged updated:>={current_iso_8601_date}");

    let mut query_parameters = HashMap::new();

    query_parameters.insert("q", search_query);
    query_parameters.insert("sort", String::from("updated"));
    query_parameters.insert("order", String::from("desc"));

    let response = client
        .get(MERGED_PRS_API_URL)
        .query(&query_parameters)
        .bearer_auth(github_token)
        .header(ACCEPT, GITHUB_ACCEPT_HEADER)
        .header(USER_AGENT, SWELOG_USER_AGENT)
        .send()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToSendGitHubRequest)?;

    let status_code = response.status();

    let response_text =
        response.text().await.into_diagnostic().wrap_err("failed to read GitHub response body")?;

    if !status_code.is_success() {
        let unsuccessful_github_response_error =
            UnsuccessfulGitHubResponse { status_code, response_text };

        return Err(unsuccessful_github_response_error.into());
    }

    let issues = parse_search_issues_response_text(&response_text)?;

    Ok(issues)
}

pub async fn get_opened_prs(github_token: &str, github_username: &str) -> Result<Vec<Issue>> {
    let client = Client::new();

    let current_iso_8601_date = get_current_date_in_iso_8601();

    let search_query = format!("author:{github_username} is:pr created:>={current_iso_8601_date}");

    let mut query_parameters = HashMap::new();

    query_parameters.insert("q", search_query);
    query_parameters.insert("sort", String::from("updated"));
    query_parameters.insert("order", String::from("desc"));

    let response = client
        .get(OPENED_PRS_API_URL)
        .query(&query_parameters)
        .bearer_auth(github_token)
        .header(ACCEPT, GITHUB_ACCEPT_HEADER)
        .header(USER_AGENT, SWELOG_USER_AGENT)
        .send()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToSendGitHubRequest)?;

    let status_code = response.status();

    let response_text =
        response.text().await.into_diagnostic().wrap_err("failed to read GitHub response body")?;

    if !status_code.is_success() {
        let unsuccessful_github_response_error =
            UnsuccessfulGitHubResponse { status_code, response_text };

        return Err(unsuccessful_github_response_error.into());
    }

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
