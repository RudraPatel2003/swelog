use miette::Result;

pub trait Llm {
    fn generate_response(&self, prompt: &str) -> Result<String>;
}
