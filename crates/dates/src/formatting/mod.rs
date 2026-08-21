#[cfg(test)]
mod tests;

use chrono::NaiveDate;

use crate::date_format::DATE_FORMAT;

/// Renders a date the way swelog writes it in log files and prompts.
#[must_use]
pub fn format_date(date: &NaiveDate) -> String {
    date.format(DATE_FORMAT).to_string()
}
