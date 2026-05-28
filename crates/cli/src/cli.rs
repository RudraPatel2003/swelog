use clap::{
    Args,
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

    /// Setup the swelog files in your Obsidian vault.
    Setup {
        /// Overwrite existing swelog files.
        #[arg(long = "force")]
        overwrite_existing_files: bool,
    },

    /// Summarize your configured work file. By default, this is an alias for `swelog summarize
    /// day`.
    Summarize {
        #[command(flatten)]
        daily_summary_options: DailySummaryOptions,

        #[command(subcommand)]
        command: Option<SummarizeCommands>,
    },

    /// Add a work item to your configured work file.
    Log {
        /// Work item to add as a Markdown bullet.
        work_item: String,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum SummarizeCommands {
    /// Summarize your configured work file and log it into the daily folder.
    Day(DailySummaryOptions),
}

#[derive(Clone, Copy, Debug, Args)]
pub(crate) struct DailySummaryOptions {
    /// Overwrite existing daily log file.
    #[arg(long = "force")]
    pub overwrite_existing_daily_log: bool,

    /// Keep the current contents of the configured work file.
    #[arg(long = "keep")]
    pub keep_work_file: bool,
}
