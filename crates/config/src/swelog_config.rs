use std::path::PathBuf;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwelogConfig {
    pub obsidian_vault_path: PathBuf,
    pub swelog_folder: PathBuf,
    pub daily_log_folder: PathBuf,
    pub weekly_log_folder: PathBuf,
}

impl SwelogConfig {
    pub fn get_default_config() -> Self {
        Self {
            obsidian_vault_path: PathBuf::from(""),
            swelog_folder: PathBuf::from("swelog"),
            daily_log_folder: PathBuf::from("daily"),
            weekly_log_folder: PathBuf::from("weekly"),
        }
    }
}
