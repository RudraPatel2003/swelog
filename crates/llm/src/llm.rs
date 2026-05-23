use async_trait::async_trait;
use miette::Result;

#[async_trait]
pub trait Llm {
    async fn generate_response(&self, prompt: &str) -> Result<String>;
}
