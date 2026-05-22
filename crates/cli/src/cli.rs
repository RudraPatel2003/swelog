use clap::{
    Parser,
    Subcommand,
};

#[derive(Debug, Parser)]
#[command(name = "swelog", version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Create a default swelog config file.
    Init {
        /// Overwrite an existing config file with defaults.
        #[arg(long = "force")]
        overwrite_existing_config: bool,
    },
}
