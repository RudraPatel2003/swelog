pub mod language_model;
pub mod language_model_factory;
pub mod ollama_language_model;
pub mod prompts;

pub use language_model::LanguageModel;
pub use language_model_factory::get_language_model_from_config;
pub use ollama_language_model::OllamaLanguageModel;
