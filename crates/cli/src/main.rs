mod cli;

use clap::Parser;
use cli::{
    Cli,
    Commands,
};
use miette::Result;
use owo_colors::OwoColorize;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { overwrite_existing_config } => {
            let config_file_path = config::initialize_config_file(overwrite_existing_config)?;

            println!("Created swelog config at {}", config_file_path.display().cyan());
        }
    }

    Ok(())
}
