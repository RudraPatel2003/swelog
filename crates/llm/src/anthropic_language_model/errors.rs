use miette::Diagnostic;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Anthropic request failed for model {model} with status {status}")]
#[diagnostic(code(swelog::llm::anthropic_request_failed))]
pub struct AnthropicRequestFailed {
    pub model: String,
    pub status: StatusCode,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Anthropic response for model {model} did not include generated text")]
#[diagnostic(code(swelog::llm::anthropic_response_missing_text))]
pub struct AnthropicResponseMissingText {
    pub model: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Anthropic rejected your API key with status {status}")]
#[diagnostic(
    code(swelog::llm::anthropic_authorization_failed),
    help("run `swelog auth clear anthropic` and run the command again to enter a new API key")
)]
pub struct AnthropicAuthorizationFailed {
    pub status: StatusCode,
}
