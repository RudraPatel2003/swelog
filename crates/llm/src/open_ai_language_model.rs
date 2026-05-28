use async_trait::async_trait;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    header::{
        AUTHORIZATION,
        HeaderValue,
    },
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    errors::{
        OpenAiRequestFailed,
        OpenAiResponseMissingText,
    },
    language_model::LanguageModel,
};

const OPEN_AI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";

pub struct OpenAiLanguageModel {
    client: Client,
    model: String,
    api_key: String,
}

impl OpenAiLanguageModel {
    pub fn new(model: String, api_key: String) -> Self {
        Self { client: Client::new(), model, api_key }
    }

    fn authorization_header(&self) -> Result<HeaderValue> {
        let mut authorization_header = HeaderValue::from_str(&format!("Bearer {}", self.api_key))
            .into_diagnostic()
            .wrap_err("failed to prepare OpenAI authorization header")?;

        authorization_header.set_sensitive(true);

        Ok(authorization_header)
    }
}

#[async_trait]
impl LanguageModel for OpenAiLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let request = OpenAiResponseRequest { model: &self.model, input: prompt };

        let response = self
            .client
            .post(OPEN_AI_RESPONSES_URL)
            .header(AUTHORIZATION, self.authorization_header()?)
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

#[derive(Serialize)]
struct OpenAiResponseRequest<'a> {
    model: &'a str,
    input: &'a str,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    #[serde(default)]
    output: Vec<OpenAiResponseOutput>,
}

#[derive(Deserialize)]
struct OpenAiResponseOutput {
    #[serde(default)]
    content: Vec<OpenAiResponseContent>,
}

#[derive(Deserialize)]
struct OpenAiResponseContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
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
mod tests {
    use super::*;

    const MODEL: &str = "gpt-5.4-mini";

    #[test]
    fn parse_open_ai_response_text_extracts_output_text() {
        let response_body = r#"
            {
              "output": [
                {
                  "type": "message",
                  "content": [
                    {
                      "type": "output_text",
                      "text": "Daily summary"
                    }
                  ]
                }
              ]
            }
        "#;

        let text = parse_open_ai_response_text(response_body, MODEL)
            .expect("OpenAI response text should parse");

        assert_eq!(text, "Daily summary");
    }

    #[test]
    fn parse_open_ai_response_text_joins_multiple_output_text_parts() {
        let response_body = r#"
            {
              "output": [
                {
                  "type": "message",
                  "content": [
                    {
                      "type": "output_text",
                      "text": "Daily "
                    },
                    {
                      "type": "output_text",
                      "text": "summary"
                    }
                  ]
                }
              ]
            }
        "#;

        let text = parse_open_ai_response_text(response_body, MODEL)
            .expect("OpenAI response text should parse");

        assert_eq!(text, "Daily summary");
    }

    #[test]
    fn parse_open_ai_response_text_fails_when_output_text_is_missing() {
        let response_body = r#"
            {
              "output": [
                {
                  "type": "message",
                  "content": []
                }
              ]
            }
        "#;

        let error = parse_open_ai_response_text(response_body, MODEL)
            .expect_err("missing output text should fail");

        let error = error
            .downcast_ref::<OpenAiResponseMissingText>()
            .expect("error should be OpenAiResponseMissingText");

        assert_eq!(error.model, MODEL);
    }
}
