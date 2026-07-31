use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize)]
pub struct OpenRouterResponseRequest {
    pub model: String,
    pub input: String,
}

#[derive(Deserialize)]
pub struct OpenRouterResponse {
    #[serde(default)]
    pub output: Vec<OpenRouterResponseOutput>,
}

#[derive(Deserialize)]
pub struct OpenRouterResponseOutput {
    #[serde(default)]
    pub content: Vec<OpenRouterResponseContent>,
}

#[derive(Deserialize)]
pub struct OpenRouterResponseContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}
