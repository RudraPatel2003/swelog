use clap::Parser;

use crate::{
    commands::Commands,
    environment::global_args::GlobalArgs,
};

#[derive(Debug, Parser)]
#[command(name = "swelog", version, about)]
pub struct Cli {
    #[command(flatten)]
    pub global_args: GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}
