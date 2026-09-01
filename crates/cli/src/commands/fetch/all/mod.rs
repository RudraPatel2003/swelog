mod errors;
mod formatting;

use chrono::NaiveDate;
use clap::Args;
use config::config_file::read_config_file;
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_date,
};
use miette::Result;

use crate::{
    commands::fetch::{
        all::{
            errors::{
                FetchSourcesFailed,
                NoConfiguredFetchSources,
            },
            formatting::{
                format_fetch_source_labels,
                format_running_notice,
            },
        },
        github::fetch_github_activity,
        google_calendar::fetch_google_calendar_meetings,
        linear::fetch_linear_issues,
        sources::{
            FetchSource,
            collect_included_fetch_sources,
        },
    },
    shared::date_selection::DateSelection,
};

#[derive(Debug, Args)]
pub struct AllArgs {
    /// Date to fetch activity for in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Fetch activity for yesterday instead of today.
    #[arg(long = "yesterday", conflicts_with = "date")]
    use_yesterday: bool,
}

impl AllArgs {
    pub async fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let included_fetch_sources = collect_included_fetch_sources(&swelog_config)?;

        if included_fetch_sources.is_empty() {
            let no_configured_fetch_sources_error = NoConfiguredFetchSources;

            return Err(no_configured_fetch_sources_error.into());
        }

        println!("{}", format_running_notice(&included_fetch_sources));

        let date_selection = DateSelection::from_date_flags(self.date, self.use_yesterday);

        let failed_fetch_sources = run_fetch_sources(&included_fetch_sources, date_selection).await;

        if failed_fetch_sources.is_empty() {
            return Ok(());
        }

        let fetch_sources_failed_error = FetchSourcesFailed {
            failed_source_labels: format_fetch_source_labels(&failed_fetch_sources),
        };

        Err(fetch_sources_failed_error.into())
    }
}

async fn run_fetch_sources(
    fetch_sources: &[FetchSource],
    date_selection: DateSelection,
) -> Vec<FetchSource> {
    let mut failed_fetch_sources = Vec::new();

    for fetch_source in fetch_sources.iter().copied() {
        println!();

        if let Err(error) = run_fetch_source(fetch_source, date_selection).await {
            eprintln!("{error:?}");

            failed_fetch_sources.push(fetch_source);
        }
    }

    failed_fetch_sources
}

async fn run_fetch_source(fetch_source: FetchSource, date_selection: DateSelection) -> Result<()> {
    match fetch_source {
        FetchSource::Github => fetch_github_activity(date_selection).await,

        FetchSource::Linear => fetch_linear_issues(date_selection).await,

        FetchSource::GoogleCalendar => fetch_google_calendar_meetings(date_selection).await,
    }
}
