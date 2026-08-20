use chrono::{
    Datelike,
    Duration,
    Local,
    NaiveDate,
    Weekday,
};
use clap::Args;
use config::utils::read_config_file;
use dates::{
    DATE_VALUE_NAME,
    parsing::parse_monday_date,
};
use llm::language_model_factory::get_language_model_from_config;
use miette::{
    Result,
    miette,
};
use owo_colors::OwoColorize;
use summary::week::{
    get_weekly_log_file_name,
    summarize_weekly_work_from_config,
};

#[derive(Debug, Args)]
pub struct WeeklySummaryArgs {
    /// The Monday of the week you want to summarize in the format MM-DD-YYYY.
    #[arg(long = "week-of", value_name = DATE_VALUE_NAME, value_parser = parse_monday_date)]
    monday_date: Option<NaiveDate>,

    /// Overwrite existing weekly log file.
    #[arg(long = "force")]
    overwrite_existing_weekly_log: bool,
}

impl WeeklySummaryArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let monday_date = match self.monday_date {
            Some(monday_date) => monday_date,
            None => get_most_recent_monday_date()?,
        };

        let language_model = get_language_model_from_config(&swelog_config)?;

        summarize_weekly_work_from_config(
            &swelog_config,
            language_model.as_ref(),
            &monday_date,
            self.overwrite_existing_weekly_log,
        )
        .await?;

        let weekly_log_file_name = get_weekly_log_file_name(&monday_date);

        println!("Succesfully summarized your weekly work into {}", weekly_log_file_name.cyan());

        Ok(())
    }
}

fn get_most_recent_monday_date() -> Result<NaiveDate> {
    let today = Local::now().date_naive();

    let days_since_monday = match today.weekday() {
        Weekday::Mon => 7, // last Monday
        weekday => i64::from(weekday.num_days_from_monday()),
    };

    today
        .checked_sub_signed(Duration::days(days_since_monday))
        .ok_or_else(|| miette!("failed to determine the Monday of the current week"))
}
