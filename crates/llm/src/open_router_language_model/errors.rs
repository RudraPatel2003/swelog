use miette::Diagnostic;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("OpenRouter request failed for model {model} with status {status}")]
#[diagnostic(code(swelog::llm::open_router_request_failed))]
pub struct OpenRouterRequestFailed {
    pub model: String,
    pub status: StatusCode,
}

#[derive(Debug, Diagnostic, Error)]
#[error("OpenRouter response for model {model} did not include generated text")]
#[diagnostic(code(swelog::llm::open_router_response_missing_text))]
pub struct OpenRouterResponseMissingText {
    pub model: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("OpenRouter rejected your API key with status {status}")]
#[diagnostic(
    code(swelog::llm::open_router_authorization_failed),
    help("run `swelog auth clear open-router` and run the command again to enter a new API key")
)]
pub struct OpenRouterAuthorizationFailed {
    pub status: StatusCode,
}
