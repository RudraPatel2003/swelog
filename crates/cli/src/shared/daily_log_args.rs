use chrono::NaiveDate;
use clap::Args;
use config::overwrite::Overwrite;
use daily_log::work_file::KeepWorkFile;
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_date,
};
use miette::Result;

use crate::shared::date_selection::{
    DateSelection,
    resolve_selected_date,
};

/// The flags shared by every command that writes a daily log.
#[derive(Debug, Args)]
pub struct DailyLogArgs {
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

impl DailyLogArgs {
    pub fn resolve_log_date(&self, today: NaiveDate) -> Result<NaiveDate> {
        let date_selection = DateSelection::from_date_flags(self.date, self.use_yesterday);

        let log_date = resolve_selected_date(date_selection, today)?.unwrap_or(today);

        Ok(log_date)
    }

    #[must_use]
    pub const fn overwrite(&self) -> Overwrite {
        Overwrite::from_force_flag(self.overwrite_existing_daily_log)
    }

    #[must_use]
    pub const fn keep_work_file(&self) -> KeepWorkFile {
        KeepWorkFile::from_keep_flag(self.keep_work_file)
    }
}
