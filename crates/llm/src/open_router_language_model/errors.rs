use miette::Diagnostic;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("OpenRouter API key is missing")]
#[diagnostic(
    code(swelog::llm::missing_open_router_api_key),
    help("set the OPENROUTER_API_KEY environment variable before running `swelog summarize`")
)]
pub struct MissingOpenRouterApiKey;

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
