use miette::Diagnostic;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to send GitHub request")]
#[diagnostic(
    code(swelog::github::failed_to_send_github_request),
    help("check your GitHub token and network connection and try again")
)]
pub struct FailedToSendGitHubRequest;

#[derive(Debug, Diagnostic, Error)]
#[error("Unsuccessful GitHub response with status {status_code} and body {response_text}")]
#[diagnostic(code(swelog::github::unsuccessful_github_response))]
pub struct UnsuccessfulGitHubResponse {
    pub status_code: StatusCode,
    pub response_text: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("GitHub rejected your token with status {status_code}")]
#[diagnostic(
    code(swelog::github::authorization_failed),
    help(
        "run `swelog auth clear github` and run the command again to enter a new token, or check that your token has the `repo` scope"
    )
)]
pub struct GitHubAuthorizationFailed {
    pub status_code: StatusCode,
}
