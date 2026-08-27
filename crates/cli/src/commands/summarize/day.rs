use chrono::Local;
use clap::Args;
use config::utils::read_config_file;
use daily_log::file::get_daily_log_file_name;
use llm::language_model_factory::get_language_model_from_config;
use miette::Result;
use owo_colors::OwoColorize;
use summary::day::summarize_daily_work_from_config;

use crate::shared::daily_log_args::DailyLogArgs;

#[derive(Debug, Args)]
pub struct DailySummaryArgs {
    #[command(flatten)]
    daily_log_args: DailyLogArgs,
}

impl DailySummaryArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let language_model = get_language_model_from_config(&swelog_config)?;

        let today = Local::now().date_naive();

        let log_date = self.daily_log_args.resolve_log_date(today)?;

        summarize_daily_work_from_config(
            &swelog_config,
            language_model.as_ref(),
            &log_date,
            self.daily_log_args.overwrite(),
            self.daily_log_args.keep_work_file(),
        )
        .await?;

        let daily_log_file_name = get_daily_log_file_name(&log_date);

        println!("Successfully summarized your daily work into {}", daily_log_file_name.cyan());

        Ok(())
    }
}
