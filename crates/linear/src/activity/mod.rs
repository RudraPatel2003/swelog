use chrono::{
    DateTime,
    Days,
    Local,
    NaiveDate,
    Utc,
};
use dates::formatting::format_date;

use crate::{
    LinearIssue,
    errors::LinearDateOutOfRange,
};

/// The [`chrono`] format string Linear expects for its timestamp filters.
const LINEAR_FILTER_DATE_FORMAT: &str = "%Y-%m-%d";

/// Builds the `updatedAt` argument that bounds how far back Linear searches.
///
/// Linear only accepts a lower bound in UTC, so ask for a full day earlier than
/// the requested date. That covers every local time zone offset, and the issues
/// it over-fetches are discarded by [`was_issue_worked_on`].
pub fn get_updated_after_filter(date: NaiveDate) -> Result<String, LinearDateOutOfRange> {
    let earliest_date = date
        .checked_sub_days(Days::new(1))
        .ok_or_else(|| LinearDateOutOfRange { date: format_date(&date) })?;

    Ok(format!("{}T00:00:00Z", earliest_date.format(LINEAR_FILTER_DATE_FORMAT)))
}

/// Decides whether an issue belongs in the work file for a date.
///
/// Linear does not expose issue history, so the only evidence available is the
/// timestamps on the issue itself. An issue counts when any of them lands on the
/// date.
pub fn was_issue_worked_on(issue: &LinearIssue, date: NaiveDate) -> bool {
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
    .any(|instant| falls_on_local_date(instant, date))
}

/// Linear records timestamps in UTC, while the requested date is the one the
/// user reads on their own calendar.
fn falls_on_local_date(instant: DateTime<Utc>, date: NaiveDate) -> bool {
    instant.with_timezone(&Local).date_naive() == date
}

#[cfg(test)]
mod tests;
