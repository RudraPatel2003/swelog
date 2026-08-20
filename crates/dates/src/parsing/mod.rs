#[cfg(test)]
mod tests;

use chrono::{
    Datelike,
    NaiveDate,
    Weekday,
};

use crate::{
    DATE_FORMAT,
    errors::DateParseError,
};

/// Reads a date written in the swelog date format.
pub fn parse_date(date: &str) -> Result<NaiveDate, DateParseError> {
    NaiveDate::parse_from_str(date, DATE_FORMAT)
        .map_err(|_| DateParseError::InvalidDate { date: date.to_string() })
}

/// Reads a date written in the swelog date format that must land on a Monday.
pub fn parse_monday_date(date: &str) -> Result<NaiveDate, DateParseError> {
    let monday_date = parse_date(date)?;

    if monday_date.weekday() != Weekday::Mon {
        return Err(DateParseError::DateIsNotAMonday { date: date.to_string() });
    }

    Ok(monday_date)
}
