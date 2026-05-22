use std::path::{
    Path,
    PathBuf,
};

use serde::{
    Deserialize,
    Serialize,
};

const APP_NAME: &str = "swelog";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SwelogConfig {
    pub obsidian_vault_path: PathBuf,
    pub swelog_folder: PathBuf,
    pub daily_log_folder: PathBuf,
    pub weekly_log_folder: PathBuf,
}

impl SwelogConfig {
    pub fn get_default_config(vault_directory: &Path) -> Self {
        let vault_path = vault_directory.join(APP_NAME);

        Self {
            obsidian_vault_path: vault_path,
            swelog_folder: PathBuf::from("swelog"),
            daily_log_folder: PathBuf::from("Daily"),
            weekly_log_folder: PathBuf::from("Weekly"),
        }
    }
}
