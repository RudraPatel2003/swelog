use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize)]
pub struct OpenAiResponseRequest {
    pub model: String,
    pub input: String,
}

#[derive(Deserialize)]
pub struct OpenAiResponse {
    #[serde(default)]
    pub output: Vec<OpenAiResponseOutput>,
}

#[derive(Deserialize)]
pub struct OpenAiResponseOutput {
    #[serde(default)]
    pub content: Vec<OpenAiResponseContent>,
}

#[derive(Deserialize)]
pub struct OpenAiResponseContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}
