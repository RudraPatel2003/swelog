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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LanguageModelProvider {
    Ollama,
    OpenAi,
    OpenRouter,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SwelogConfig {
    pub obsidian_vault_path: PathBuf,
    pub swelog_folder_name: String,
    pub work_file_name: String,
    pub context_file_name: String,
    pub daily_log_folder_name: String,
    pub weekly_log_folder_name: String,

    #[serde(default, rename = "llm", skip_serializing_if = "Option::is_none")]
    pub language_model_provider: Option<LanguageModelProvider>,

    #[serde(default, rename = "llmModel", skip_serializing_if = "Option::is_none")]
    pub language_model: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear_username: Option<String>,
}

impl SwelogConfig {
    #[must_use]
    pub fn get_default_config() -> Self {
        Self {
            obsidian_vault_path: PathBuf::from(DEFAULT_OBSIDIAN_VAULT_PATH),
            swelog_folder_name: String::from(DEFAULT_SWELOG_FOLDER_NAME),
            work_file_name: String::from(DEFAULT_WORK_FILE_NAME),
            context_file_name: String::from(DEFAULT_CONTEXT_FILE_NAME),
            daily_log_folder_name: String::from(DEFAULT_DAILY_LOG_FOLDER_NAME),
            weekly_log_folder_name: String::from(DEFAULT_WEEKLY_LOG_FOLDER_NAME),
            language_model_provider: None,
            language_model: None,
            linear_username: None,
        }
    }
}
