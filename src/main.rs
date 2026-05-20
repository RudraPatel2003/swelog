mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "swelog", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a default swelog config file.
    Init {
        /// Overwrite an existing config file with defaults.
        #[arg(long = "force")]
        overwrite_existing_config: bool,
    },
}

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
