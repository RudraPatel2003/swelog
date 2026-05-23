use async_trait::async_trait;
use config::swelog_config::{
    SupportedLlm,
    SwelogConfig,
};
use miette::Result;

use crate::{
    llm::Llm,
    ollama::OllamaLlm,
};

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

pub fn get_llm_provider_from_config(swelog_config: &SwelogConfig) -> LlmProvider {
    match &swelog_config.llm {
        SupportedLlm::Ollama => {
            let ollama_model = swelog_config.ollama_model.clone();

            LlmProvider::Ollama(OllamaLlm::new(ollama_model))
        }
    }
}
