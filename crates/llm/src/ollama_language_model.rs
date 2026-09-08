use async_trait::async_trait;
use base_url::base_url::BaseUrl;
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

pub const DEFAULT_OLLAMA_BASE_URL: &str = "http://localhost:11434/";

pub struct OllamaLanguageModel {
    client: Ollama,
    model: String,
}

impl OllamaLanguageModel {
    pub fn new(base_url: &BaseUrl, model: String) -> Result<Self> {
        let client = Ollama::try_new(base_url.as_str())
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to connect to Ollama at {base_url}"))?;

        Ok(Self { client, model })
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
