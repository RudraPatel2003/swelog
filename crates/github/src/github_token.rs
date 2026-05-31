use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::errors::MissingGitHubToken;

const GITHUB_TOKEN_ENVIRONMENT_VARIABLE: &str = "GITHUB_TOKEN";

pub fn get_github_token() -> Result<String> {
    let github_token = std::env::var(GITHUB_TOKEN_ENVIRONMENT_VARIABLE)
        .into_diagnostic()
        .wrap_err_with(|| MissingGitHubToken)?;

    Ok(github_token)
}
