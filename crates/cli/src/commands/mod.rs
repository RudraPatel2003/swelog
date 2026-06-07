mod config;
mod fetch;
mod init;
mod log;
mod reset;
mod setup;
mod summarize;

use clap::Subcommand;
use fetch::FetchArgs;
use init::InitArgs;
use log::LogArgs;
use reset::ResetArgs;
use setup::SetupArgs;
use summarize::SummarizeArgs;

use self::config::ConfigArgs;

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

    /// Reset your work file to the default content.
    Reset(ResetArgs),

    /// Add data from external sources to your work file.
    Fetch(FetchArgs),

    /// Display your current swelog configuration and where it is stored.
    Config(ConfigArgs),
}
