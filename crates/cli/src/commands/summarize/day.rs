use chrono::{
    Local,
    NaiveDate,
};
use clap::Args;
use config::{
    overwrite::Overwrite,
    utils::read_config_file,
};
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_date,
};
use llm::language_model_factory::get_language_model_from_config;
use miette::Result;
use owo_colors::OwoColorize;
use summary::day::{
    KeepWorkFile,
    get_daily_log_file_name,
    summarize_daily_work_from_config,
};

use crate::commands::date_selection::{
    DateSelection,
    resolve_selected_date,
};

#[derive(Debug, Args)]
pub struct DailySummaryArgs {
    /// Date to write the daily log for in the format MM-DD-YYYY. Defaults to today.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Write the daily log for yesterday instead of today.
    #[arg(long = "yesterday", conflicts_with = "date")]
    use_yesterday: bool,

    /// Overwrite existing daily log file.
    #[arg(long = "force")]
    overwrite_existing_daily_log: bool,

    /// Keep the current contents of the configured work file.
    #[arg(long = "keep")]
    keep_work_file: bool,
}

impl DailySummaryArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let language_model = get_language_model_from_config(&swelog_config)?;

        let today = Local::now().date_naive();

        let date_selection = DateSelection::from_date_flags(self.date, self.use_yesterday);

        let log_date = resolve_selected_date(date_selection, today)?.unwrap_or(today);

        summarize_daily_work_from_config(
            &swelog_config,
            language_model.as_ref(),
            &log_date,
            Overwrite::from_force_flag(self.overwrite_existing_daily_log),
            KeepWorkFile::from_keep_flag(self.keep_work_file),
        )
        .await?;

        let daily_log_file_name = get_daily_log_file_name(&log_date);

        println!("Successfully summarized your daily work into {}", daily_log_file_name.cyan());

        Ok(())
    }
}
