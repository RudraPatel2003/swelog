use serde::{
    Deserialize,
    Serialize,
};
use std::path::{
    Path,
    PathBuf,
};

const APP_NAME: &str = "swelog";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SwelogConfig {
    pub obsidian_vault_path: PathBuf,
    pub daily_log_folder: PathBuf,
    pub weekly_log_folder: PathBuf,
    pub monthly_log_folder: PathBuf,
    pub quarterly_log_folder: PathBuf,
    pub yearly_log_folder: PathBuf,
}

impl SwelogConfig {
    pub fn get_default_config(vault_directory: &Path) -> Self {
        let vault_path = vault_directory.join(APP_NAME);

        Self {
            obsidian_vault_path: vault_path,
            daily_log_folder: PathBuf::from("swelog/daily"),
            weekly_log_folder: PathBuf::from("swelog/weekly"),
            monthly_log_folder: PathBuf::from("swelog/monthly"),
            quarterly_log_folder: PathBuf::from("swelog/quarterly"),
            yearly_log_folder: PathBuf::from("swelog/yearly"),
        }
    }
}
