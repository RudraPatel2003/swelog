use clap::Args;
use config::{
    setup::setup_swelog_files_from_config,
    utils::read_config_file,
};
use miette::Result;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct SetupArgs {
    /// Overwrite existing swelog files.
    #[arg(long = "force")]
    overwrite_existing_files: bool,
}

impl SetupArgs {
    pub fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        setup_swelog_files_from_config(&swelog_config, self.overwrite_existing_files)?;

        println!(
            "Created swelog files in your Obsidian vault at {}",
            swelog_config.obsidian_vault_path.display().cyan()
        );

        Ok(())
    }
}
