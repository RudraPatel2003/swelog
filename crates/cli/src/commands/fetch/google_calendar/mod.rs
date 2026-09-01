mod formatting;

use chrono::{
    Local,
    NaiveDate,
};
use clap::Args;
use config::config_file::read_config_file;
use dates::{
    date_format::DATE_VALUE_NAME,
    formatting::format_date,
    parsing::parse_date,
};
use google_calendar::client::get_primary_calendar_meetings_on_date;
use markdown::work_file::{
    remove_work_file_section_from_config,
    upsert_work_file_section_from_config,
};
use miette::Result;

use crate::{
    commands::fetch::google_calendar::formatting::format_meetings,
    shared::date_selection::{
        DateSelection,
        resolve_selected_date,
    },
};

const GOOGLE_CALENDAR_SECTION_TITLE: &str = "Google Calendar";

#[derive(Debug, Args)]
pub struct GoogleCalendarArgs {
    /// Date to fetch Google Calendar meetings for in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Fetch the meetings you had yesterday.
    #[arg(long = "yesterday", conflicts_with = "date")]
    use_yesterday: bool,
}

impl GoogleCalendarArgs {
    pub async fn run(self) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(self.date, self.use_yesterday);

        fetch_google_calendar_meetings(date_selection).await
    }
}

pub async fn fetch_google_calendar_meetings(date_selection: DateSelection) -> Result<()> {
    let swelog_config = read_config_file()?;

    let today = Local::now().date_naive();

    let meeting_date = resolve_selected_date(date_selection, today)?.unwrap_or(today);

    println!("Fetching Google Calendar events...");

    let meetings = get_primary_calendar_meetings_on_date(&meeting_date).await?;

    if meetings.is_empty() {
        remove_work_file_section_from_config(&swelog_config, GOOGLE_CALENDAR_SECTION_TITLE)?;

        println!("No meetings found for {}.", format_date(&meeting_date));

        return Ok(());
    }

    upsert_work_file_section_from_config(
        &swelog_config,
        GOOGLE_CALENDAR_SECTION_TITLE,
        &format_meetings(&meetings),
    )?;

    println!(
        "Added {} meetings from {} to your work file.",
        meetings.len(),
        format_date(&meeting_date)
    );

    Ok(())
}
