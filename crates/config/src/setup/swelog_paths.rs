use std::path::PathBuf;

use crate::swelog_config::SwelogConfig;

pub struct SwelogPaths {
    pub swelog_directory: PathBuf,
    pub context_file: PathBuf,
    pub work_file: PathBuf,
    pub daily_log_directory: PathBuf,
    pub weekly_log_directory: PathBuf,
}

impl SwelogPaths {
    pub fn new(swelog_config: &SwelogConfig) -> Self {
        let swelog_directory =
            swelog_config.obsidian_vault_path.join(&swelog_config.swelog_folder_name);

        Self {
            context_file: swelog_directory.join(&swelog_config.context_file_name),
            work_file: swelog_directory.join(&swelog_config.work_file_name),
            daily_log_directory: swelog_directory.join(&swelog_config.daily_log_folder_name),
            weekly_log_directory: swelog_directory.join(&swelog_config.weekly_log_folder_name),
            swelog_directory,
        }
    }

    pub fn all_paths(&self) -> [&PathBuf; 4] {
        [&self.context_file, &self.work_file, &self.daily_log_directory, &self.weekly_log_directory]
    }
}
