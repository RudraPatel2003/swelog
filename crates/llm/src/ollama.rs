use async_trait::async_trait;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use ollama_rs::{
    Ollama,
    generation::completion::request::GenerationRequest,
};

use crate::llm::Llm;

pub struct OllamaLlm {
    client: Ollama,
    model: String,
}

impl OllamaLlm {
    pub fn new(model: &String) -> Self {
        Self { client: Ollama::default(), model }
    }
}

#[async_trait]
impl Llm for OllamaLlm {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let request = GenerationRequest::new(self.model.clone(), prompt);

        let response = self
            .client
            .generate(request)
            .await
            .into_diagnostic()
            .wrap_err("failed to generate response with Ollama")?;

        Ok(response.response)
    }
}
