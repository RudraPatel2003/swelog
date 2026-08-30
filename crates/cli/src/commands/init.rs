use clap::Args;
use config::{
    config_file::get_config_file_path,
    init::write_default_config,
    overwrite::Overwrite,
    swelog_config::SwelogConfig,
};
use miette::Result;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Overwrite an existing config file with defaults.
    #[arg(long = "force")]
    overwrite_existing_config: bool,
}

impl InitArgs {
    pub fn run(self) -> Result<()> {
        let default_config = SwelogConfig::get_default_config();

        let config_file_path = get_config_file_path()?;

        write_default_config(
            &config_file_path,
            &default_config,
            Overwrite::from_force_flag(self.overwrite_existing_config),
        )?;

        println!("Created swelog config at {}", config_file_path.display().cyan());

        Ok(())
    }
}
