use config::swelog_config::{
    LanguageModelProvider,
    SwelogConfig,
};
use credentials::{
    Credential,
    get_or_prompt_for_credential,
};
use miette::Result;

use crate::{
    language_model::LanguageModel,
    ollama_language_model::OllamaLanguageModel,
    open_ai_language_model::OpenAiLanguageModel,
    open_router_language_model::OpenRouterLanguageModel,
};

pub fn get_language_model_from_config(
    swelog_config: &SwelogConfig,
) -> Result<Box<dyn LanguageModel>> {
    let language_model = swelog_config.language_model.clone();

    match &swelog_config.language_model_provider {
        LanguageModelProvider::Ollama => {
            let ollama_language_model = OllamaLanguageModel::new(language_model);

            Ok(Box::new(ollama_language_model))
        }

        LanguageModelProvider::OpenAi => {
            let open_ai_api_key = get_or_prompt_for_credential(Credential::OpenAi)?;

            let open_ai_language_model = OpenAiLanguageModel::new(language_model, open_ai_api_key);

            Ok(Box::new(open_ai_language_model))
        }

        LanguageModelProvider::OpenRouter => {
            let open_router_api_key = get_or_prompt_for_credential(Credential::OpenRouter)?;

            let open_router_language_model =
                OpenRouterLanguageModel::new(language_model, open_router_api_key);

            Ok(Box::new(open_router_language_model))
        }
    }
}
