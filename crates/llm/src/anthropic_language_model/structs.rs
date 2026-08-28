use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize)]
pub struct AnthropicMessagesRequest {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<AnthropicRequestMessage>,
}

#[derive(Serialize)]
pub struct AnthropicRequestMessage {
    pub role: &'static str,
    pub content: String,
}

#[derive(Deserialize)]
pub struct AnthropicResponse {
    #[serde(default)]
    pub content: Vec<AnthropicResponseContent>,
}

#[derive(Deserialize)]
pub struct AnthropicResponseContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}
