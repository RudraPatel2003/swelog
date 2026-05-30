use chrono::Local;
use clap::{
    Args,
    Subcommand,
};
use config::utils::read_config_file;
use llm::language_model_factory::get_language_model_from_config;
use miette::Result;
use owo_colors::OwoColorize;
use summary::day::{
    get_daily_log_file_name,
    summarize_daily_work_from_config,
};

#[derive(Debug, Args)]
pub struct SummarizeArgs {
    #[command(flatten)]
    daily_summary_options: DailySummaryOptions,

    #[command(subcommand)]
    command: Option<SummarizeCommands>,
}

#[derive(Clone, Copy, Debug, Args)]
struct DailySummaryOptions {
    /// Overwrite existing daily log file.
    #[arg(long = "force")]
    overwrite_existing_daily_log: bool,

    /// Keep the current contents of the configured work file.
    #[arg(long = "keep")]
    keep_work_file: bool,
}

#[derive(Debug, Subcommand)]
enum SummarizeCommands {
    /// Summarize your configured work file and log it into the daily folder.
    Day(DailySummaryOptions),
}

impl SummarizeArgs {
    pub async fn run(self) -> Result<()> {
        let daily_summary_options = match self.command {
            Some(SummarizeCommands::Day(daily_summary_options)) => daily_summary_options,
            None => self.daily_summary_options,
        };

        let swelog_config = read_config_file()?;

        let language_model = get_language_model_from_config(&swelog_config)?;

        let log_date = Local::now().date_naive();

        summarize_daily_work_from_config(
            &swelog_config,
            language_model.as_ref(),
            &log_date,
            daily_summary_options.overwrite_existing_daily_log,
            daily_summary_options.keep_work_file,
        )
        .await?;

        let daily_log_file_name = get_daily_log_file_name(&log_date);

        println!("Successfully summarized your daily work into {}", daily_log_file_name.cyan());

        Ok(())
    }
}
