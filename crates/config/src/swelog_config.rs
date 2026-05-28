use std::path::PathBuf;

use serde::{
    Deserialize,
    Serialize,
};

const DEFAULT_OBSIDIAN_VAULT_PATH: &str = "";
pub const DEFAULT_SWELOG_FOLDER_NAME: &str = "swelog";
pub const DEFAULT_DAILY_LOG_FOLDER_NAME: &str = "Daily";
pub const DEFAULT_WEEKLY_LOG_FOLDER_NAME: &str = "Weekly";
pub const DEFAULT_WORK_FILE_NAME: &str = "WORK.md";
pub const DEFAULT_CONTEXT_FILE_NAME: &str = "CONTEXT.md";
pub const DEFAULT_OLLAMA_MODEL: &str = "llama3.2";
pub const DEFAULT_OPEN_AI_MODEL: &str = "gpt-5.4-mini";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LanguageModelProvider {
    Ollama,
    OpenAi,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwelogConfig {
    pub obsidian_vault_path: PathBuf,
    pub swelog_folder_name: String,
    pub work_file_name: String,
    pub context_file_name: String,
    pub daily_log_folder_name: String,
    pub weekly_log_folder_name: String,

    #[serde(rename = "llm")]
    pub language_model_provider: LanguageModelProvider,

    pub ollama_model: String,

    #[serde(default = "default_open_ai_model")]
    pub open_ai_model: String,
}

impl SwelogConfig {
    pub fn get_default_config() -> Self {
        Self {
            obsidian_vault_path: PathBuf::from(DEFAULT_OBSIDIAN_VAULT_PATH),
            swelog_folder_name: String::from(DEFAULT_SWELOG_FOLDER_NAME),
            work_file_name: String::from(DEFAULT_WORK_FILE_NAME),
            context_file_name: String::from(DEFAULT_CONTEXT_FILE_NAME),
            daily_log_folder_name: String::from(DEFAULT_DAILY_LOG_FOLDER_NAME),
            weekly_log_folder_name: String::from(DEFAULT_WEEKLY_LOG_FOLDER_NAME),
            language_model_provider: LanguageModelProvider::Ollama,
            ollama_model: String::from(DEFAULT_OLLAMA_MODEL),
            open_ai_model: default_open_ai_model(),
        }
    }
}

fn default_open_ai_model() -> String {
    String::from(DEFAULT_OPEN_AI_MODEL)
}

#[cfg(test)]
mod tests;
