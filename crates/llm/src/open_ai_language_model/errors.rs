use miette::Diagnostic;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("OpenAI request failed for model {model} with status {status}")]
#[diagnostic(code(swelog::llm::open_ai_request_failed))]
pub struct OpenAiRequestFailed {
    pub model: String,
    pub status: StatusCode,
}

#[derive(Debug, Diagnostic, Error)]
#[error("OpenAI response for model {model} did not include generated text")]
#[diagnostic(code(swelog::llm::open_ai_response_missing_text))]
pub struct OpenAiResponseMissingText {
    pub model: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("OpenAI rejected your API key with status {status}")]
#[diagnostic(
    code(swelog::llm::open_ai_authorization_failed),
    help("run `swelog auth clear open-ai` and run the command again to enter a new API key")
)]
pub struct OpenAiAuthorizationFailed {
    pub status: StatusCode,
}
