use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("GitHub Token is missing")]
#[diagnostic(
    code(swelog::github::missing_github_token),
    help("set the GITHUB_TOKEN environment variable before running `swelog summarize`")
)]
pub struct MissingGitHubToken;

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to send GitHub request")]
#[diagnostic(
    code(swelog::github::failed_to_send_github_request),
    help("check your GitHub token and network connection and try again")
)]
pub struct FailedToSendGitHubRequest;
