use chrono::{
    Datelike,
    Duration,
    Local,
    NaiveDate,
    Weekday,
};
use clap::Args;
use config::utils::read_config_file;
use llm::language_model_factory::get_language_model_from_config;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
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
    #[arg(long = "week-of")]
    monday_date_string: Option<String>,

    /// Overwrite existing weekly log file.
    #[arg(long = "force")]
    overwrite_existing_weekly_log: bool,
}

impl WeeklySummaryArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let monday_date_string = get_monday_date_string(self.monday_date_string)?;

        let monday_date = parse_monday_date_string(&monday_date_string)?;

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

fn get_monday_date_string(provided_monday_date: Option<String>) -> Result<String> {
    if let Some(provided_monday_date) = provided_monday_date {
        return Ok(provided_monday_date);
    }

    let today = Local::now().date_naive();

    let days_since_monday = match today.weekday() {
        Weekday::Mon => 7, // last Monday
        weekday => weekday.num_days_from_monday() as i64,
    };

    let monday_date = today - Duration::days(days_since_monday);

    let monday_date_string = monday_date.format("%m-%d-%Y").to_string();

    Ok(monday_date_string)
}

fn parse_monday_date_string(monday_date: &str) -> Result<NaiveDate> {
    let monday_date = NaiveDate::parse_from_str(monday_date, "%m-%d-%Y")
        .into_diagnostic()
        .wrap_err_with(|| {
            format!(
                "failed to parse week start date: {}. Please use the format MM-DD-YYYY",
                monday_date
            )
        })?;

    if monday_date.weekday() != Weekday::Mon {
        return Err(miette!("Week start date must be a Monday. Please use the format MM-DD-YYYY"));
    }

    Ok(monday_date)
}
