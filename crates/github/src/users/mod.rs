mod structs;

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
use structs::UserResponse;

use crate::{
    errors::FailedToSendGitHubRequest,
    response::read_successful_response_body,
    utils::{
        GITHUB_ACCEPT_HEADER,
        SWELOG_USER_AGENT,
    },
};

const USER_API_URL: &str = "https://api.github.com/user";

pub async fn get_github_username(github_token: &str) -> Result<String> {
    let client = Client::new();

    let response = client
        .get(USER_API_URL)
        .bearer_auth(github_token)
        .header(ACCEPT, GITHUB_ACCEPT_HEADER)
        .header(USER_AGENT, SWELOG_USER_AGENT)
        .send()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToSendGitHubRequest)?;

    let response_text = read_successful_response_body(response).await?;

    let username = parse_user_response_text(&response_text)?;

    Ok(username)
}

fn parse_user_response_text(response_text: &str) -> Result<String> {
    let user_response: UserResponse = serde_json::from_str(response_text)
        .into_diagnostic()
        .wrap_err("failed to parse GitHub user response")?;

    Ok(user_response.login)
}

#[cfg(test)]
mod tests;
