pub mod errors;
mod structs;

use async_trait::async_trait;
use errors::{
    OpenAiRequestFailed,
    OpenAiResponseMissingText,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::Client;
use structs::{
    OpenAiResponse,
    OpenAiResponseRequest,
};

use crate::language_model::LanguageModel;

const OPEN_AI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";

pub struct OpenAiLanguageModel {
    client: Client,
    model: String,
    api_key: String,
}

impl OpenAiLanguageModel {
    #[must_use]
    pub fn new(model: String, api_key: String) -> Self {
        Self { client: Client::new(), model, api_key }
    }
}

#[async_trait]
impl LanguageModel for OpenAiLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let request =
            OpenAiResponseRequest { model: self.model.clone(), input: String::from(prompt) };

        let response = self
            .client
            .post(OPEN_AI_RESPONSES_URL)
            .bearer_auth(self.api_key.clone())
            .json(&request)
            .send()
            .await
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to send OpenAI request for model {}", self.model))?;

        if !response.status().is_success() {
            let open_ai_request_failed_error =
                OpenAiRequestFailed { model: self.model.clone(), status: response.status() };

            return Err(open_ai_request_failed_error.into());
        }

        let response_body = response
            .text()
            .await
            .into_diagnostic()
            .wrap_err("failed to read OpenAI response body")?;

        parse_open_ai_response_text(&response_body, &self.model)
    }
}

fn parse_open_ai_response_text(response_body: &str, model: &str) -> Result<String> {
    let open_ai_response: OpenAiResponse = serde_json::from_str(response_body)
        .into_diagnostic()
        .wrap_err("failed to parse OpenAI response")?;

    let text_parts: Vec<&str> = open_ai_response
        .output
        .iter()
        .flat_map(|output| output.content.iter())
        .filter(|content| content.content_type == "output_text")
        .filter_map(|content| content.text.as_deref())
        .collect();

    if text_parts.is_empty() {
        let open_ai_response_missing_text_error =
            OpenAiResponseMissingText { model: model.to_string() };

        return Err(open_ai_response_missing_text_error.into());
    }

    Ok(text_parts.join(""))
}

#[cfg(test)]
mod tests;
