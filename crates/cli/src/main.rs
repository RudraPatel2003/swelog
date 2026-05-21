mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{
    Cli,
    Commands,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            overwrite_existing_config,
        } => {
            config::initialize_config_file(overwrite_existing_config)?;
        }
    }

    Ok(())
}
