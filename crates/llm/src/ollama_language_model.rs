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

use crate::language_model::LanguageModel;

pub struct OllamaLanguageModel {
    client: Ollama,
    model: String,
}

impl OllamaLanguageModel {
    pub fn new(model: String) -> Self {
        Self { client: Ollama::default(), model }
    }
}

#[async_trait]
impl LanguageModel for OllamaLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let request = GenerationRequest::new(self.model.clone(), prompt);

        let response =
            self.client.generate(request).await.into_diagnostic().wrap_err_with(|| {
                format!("Failed to generate response with Ollama model {}. Please ensure the model is available and running.", self.model)
            })?;

        Ok(response.response)
    }
}
