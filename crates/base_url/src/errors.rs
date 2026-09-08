use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("`{value}` is not a valid base URL: {message}")]
#[diagnostic(
    code(swelog::base_url::invalid_base_url),
    help("use an absolute http or https URL such as https://api.github.com/")
)]
pub struct InvalidBaseUrl {
    pub value: String,
    pub message: String,
}
