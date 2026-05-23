mod llm;
mod ollama;

pub mod prompts;

use async_trait::async_trait;
use config::swelog_config::{
    SupportedLlm,
    SwelogConfig,
};
pub use llm::Llm;
use miette::Result;
pub use ollama::OllamaLlm;

pub enum LlmProvider {
    Ollama(OllamaLlm),
}

#[async_trait]
impl Llm for LlmProvider {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        match self {
            Self::Ollama(llm) => llm.generate_response(prompt).await,
        }
    }
}

pub fn from_config(swelog_config: &SwelogConfig) -> LlmProvider {
    match &swelog_config.llm {
        SupportedLlm::Ollama => LlmProvider::Ollama(OllamaLlm::new(swelog_config.ollama_model)),
    }
}
