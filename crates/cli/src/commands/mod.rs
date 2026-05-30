mod init;
mod log;
mod setup;
mod summarize;

use clap::Subcommand;
use init::InitArgs;
use log::LogArgs;
use setup::SetupArgs;
use summarize::SummarizeArgs;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a default swelog config file.
    Init(InitArgs),

    /// Setup the swelog files in your Obsidian vault.
    Setup(SetupArgs),

    /// Summarize your configured work file. By default, this is an alias for `swelog summarize
    /// day`.
    Summarize(SummarizeArgs),

    /// Add a work item to your configured work file.
    Log(LogArgs),
}
