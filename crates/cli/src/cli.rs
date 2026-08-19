use clap::Parser;

use crate::commands::Commands;

#[derive(Debug, Parser)]
#[command(name = "swelog", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
