pub mod errors;
mod structs;

use async_trait::async_trait;
use errors::{
    OpenRouterAuthorizationFailed,
    OpenRouterRequestFailed,
    OpenRouterResponseMissingText,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    StatusCode,
};
use structs::{
    OpenRouterResponse,
    OpenRouterResponseRequest,
};

use crate::language_model::LanguageModel;

const OPEN_ROUTER_RESPONSES_URL: &str = "https://openrouter.ai/api/v1/responses";

pub struct OpenRouterLanguageModel {
    client: Client,
    model: String,
    api_key: String,
}

impl OpenRouterLanguageModel {
    #[must_use]
    pub fn new(model: String, api_key: String) -> Self {
        Self { client: Client::new(), model, api_key }
    }
}

#[async_trait]
impl LanguageModel for OpenRouterLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let request =
            OpenRouterResponseRequest { model: self.model.clone(), input: String::from(prompt) };

        let response = self
            .client
            .post(OPEN_ROUTER_RESPONSES_URL)
            .bearer_auth(self.api_key.clone())
            .json(&request)
            .send()
            .await
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("failed to send OpenRouter request for model {}", self.model)
            })?;

        let status = response.status();

        if matches!(status, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) {
            let open_router_authorization_failed_error = OpenRouterAuthorizationFailed { status };

            return Err(open_router_authorization_failed_error.into());
        }

        if !status.is_success() {
            let open_router_request_failed_error =
                OpenRouterRequestFailed { model: self.model.clone(), status };

            return Err(open_router_request_failed_error.into());
        }

        let response_body = response
            .text()
            .await
            .into_diagnostic()
            .wrap_err("failed to read OpenRouter response body")?;

        parse_open_router_response_text(&response_body, &self.model)
    }
}

fn parse_open_router_response_text(response_body: &str, model: &str) -> Result<String> {
    let open_router_response: OpenRouterResponse = serde_json::from_str(response_body)
        .into_diagnostic()
        .wrap_err("failed to parse OpenRouter response")?;

    let text_parts: Vec<&str> = open_router_response
        .output
        .iter()
        .flat_map(|output| output.content.iter())
        .filter(|content| content.content_type == "output_text")
        .filter_map(|content| content.text.as_deref())
        .collect();

    if text_parts.is_empty() {
        let open_router_response_missing_text_error =
            OpenRouterResponseMissingText { model: model.to_string() };

        return Err(open_router_response_missing_text_error.into());
    }

    Ok(text_parts.join(""))
}

#[cfg(test)]
mod tests;
