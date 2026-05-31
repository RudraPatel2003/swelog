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
use structs::UserResponse;

use crate::{
    errors::FailedToSendGitHubRequest,
    utils::GITHUB_ACCEPT_HEADER,
};

const USER_API_URL: &str = "https://api.github.com/user";

pub async fn get_github_username(github_token: &str) -> Result<String> {
    let client = Client::new();

    let response = client
        .get(USER_API_URL)
        .bearer_auth(github_token)
        .header(ACCEPT, GITHUB_ACCEPT_HEADER)
        .send()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToSendGitHubRequest)?;

    let response_text =
        response.text().await.into_diagnostic().wrap_err("failed to read GitHub response body")?;

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
