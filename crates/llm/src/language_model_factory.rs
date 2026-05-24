use config::swelog_config::{
    LanguageModelProvider,
    SwelogConfig,
};

use crate::{
    language_model::LanguageModel,
    ollama_language_model::OllamaLanguageModel,
};

pub fn get_language_model_from_config(swelog_config: &SwelogConfig) -> Box<dyn LanguageModel> {
    match &swelog_config.language_model_provider {
        LanguageModelProvider::Ollama => {
            let ollama_model = swelog_config.ollama_model.clone();

            let ollama_language_model = OllamaLanguageModel::new(ollama_model);

            Box::new(ollama_language_model)
        }
    }
}
