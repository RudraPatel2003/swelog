use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("summarization is not configured")]
#[diagnostic(
    code(swelog::llm::summarization_not_configured),
    help(
        "add `llm` and `llmModel` to your swelog config, or run `swelog log` to write the daily \
         log without a language model. See https://swelog.rudrapatel.net/summarization/"
    )
)]
pub struct SummarizationNotConfigured;
