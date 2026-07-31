use std::env;

use config::swelog_config::{LanguageModelProvider, SwelogConfig};
use miette::Result;

use crate::{
    language_model::LanguageModel,
    ollama_language_model::OllamaLanguageModel,
    open_ai_language_model::{OpenAiLanguageModel, errors::MissingOpenAiApiKey},
    open_router_language_model::{OpenRouterLanguageModel, errors::MissingOpenRouterApiKey},
};

const OPEN_AI_API_KEY_ENVIRONMENT_VARIABLE: &str = "OPENAI_API_KEY";
const OPEN_ROUTER_API_KEY_ENVIRONMENT_VARIABLE: &str = "OPENROUTER_API_KEY";

pub fn get_language_model_from_config(
    swelog_config: &SwelogConfig,
) -> Result<Box<dyn LanguageModel>> {
    match &swelog_config.language_model_provider {
        LanguageModelProvider::Ollama => {
            let ollama_model = swelog_config.ollama_model.clone();

            let ollama_language_model = OllamaLanguageModel::new(ollama_model);

            Ok(Box::new(ollama_language_model))
        }

        LanguageModelProvider::OpenAi => {
            let open_ai_model = swelog_config.open_ai_model.clone();

            let open_ai_api_key =
                env::var(OPEN_AI_API_KEY_ENVIRONMENT_VARIABLE).map_err(|_| MissingOpenAiApiKey)?;

            let open_ai_language_model = OpenAiLanguageModel::new(open_ai_model, open_ai_api_key);

            Ok(Box::new(open_ai_language_model))
        }

        LanguageModelProvider::OpenRouter => {
            let open_router_model = swelog_config.open_router_model.clone();

            let open_router_api_key = env::var(OPEN_ROUTER_API_KEY_ENVIRONMENT_VARIABLE)
                .map_err(|_| MissingOpenRouterApiKey)?;

            let open_router_language_model =
                OpenRouterLanguageModel::new(open_router_model, open_router_api_key);

            Ok(Box::new(open_router_language_model))
        }
    }
}
