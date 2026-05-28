use miette::Diagnostic;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("OpenAI API key is missing")]
#[diagnostic(
    code(swelog::llm::missing_open_ai_api_key),
    help("set the OPENAI_API_KEY environment variable before running `swelog summarize`")
)]
pub struct MissingOpenAiApiKey;

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
