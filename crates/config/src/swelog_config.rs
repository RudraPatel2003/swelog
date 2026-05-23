use std::path::PathBuf;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SupportedLlm {
    Ollama,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwelogConfig {
    pub obsidian_vault_path: PathBuf,
    pub swelog_folder_name: String,
    pub daily_log_folder_name: String,
    pub weekly_log_folder_name: String,

    pub llm: SupportedLlm,
    #[serde(default = "default_ollama_model")]
    pub ollama_model: String,
}

impl SwelogConfig {
    pub fn get_default_config() -> Self {
        Self {
            obsidian_vault_path: PathBuf::from(""),
            swelog_folder_name: String::from("swelog"),
            daily_log_folder_name: String::from("daily"),
            weekly_log_folder_name: String::from("weekly"),
            llm: SupportedLlm::Ollama,
            ollama_model: default_ollama_model(),
        }
    }
}

fn default_ollama_model() -> String {
    String::from("llama3.2")
}
