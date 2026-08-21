use chrono::{
    DateTime,
    Days,
    NaiveDate,
    TimeZone,
    Utc,
};
use dates::formatting::format_date;

use crate::{
    errors::LinearDateOutOfRange,
    issue::LinearIssue,
};

const LINEAR_FILTER_DATE_FORMAT: &str = "%Y-%m-%d";

/// Builds the `updatedAt` argument that bounds how far back Linear searches.
///
/// Linear accepts only a UTC lower bound, so this asks for a full day earlier to
/// cover every time zone offset. The issues that over-fetches are discarded by
/// [`was_issue_worked_on`].
pub fn get_updated_after_filter(date: NaiveDate) -> Result<String, LinearDateOutOfRange> {
    let earliest_date = date
        .checked_sub_days(Days::new(1))
        .ok_or_else(|| LinearDateOutOfRange { date: format_date(&date) })?;

    Ok(format!("{}T00:00:00Z", earliest_date.format(LINEAR_FILTER_DATE_FORMAT)))
}

/// Whether an issue belongs in the work file for a date, read in `timezone`.
///
/// Linear does not expose issue history, so the timestamps on the issue itself
/// are the only evidence available.
pub fn was_issue_worked_on(issue: &LinearIssue, date: NaiveDate, timezone: &impl TimeZone) -> bool {
    let timestamps = issue.timestamps;

    [
        timestamps.updated_at,
        timestamps.completed_at,
        timestamps.canceled_at,
        timestamps.started_at,
        timestamps.created_at,
    ]
    .into_iter()
    .flatten()
    .any(|instant| falls_on_date(instant, date, timezone))
}

fn falls_on_date(instant: DateTime<Utc>, date: NaiveDate, timezone: &impl TimeZone) -> bool {
    instant.with_timezone(timezone).date_naive() == date
}

#[cfg(test)]
mod tests;
