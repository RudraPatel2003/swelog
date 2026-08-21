use chrono::{
    Datelike,
    Duration,
    NaiveDate,
};
use miette::{
    Result,
    miette,
};

const DAYS_IN_WEEK: i64 = 7;

/// Which day a command was asked to act on. clap rejects `--date` together with
/// `--yesterday`, so these stay mutually exclusive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateSelection {
    /// Neither flag was passed, so the command applies its own default.
    Unspecified,
    Yesterday,
    On(NaiveDate),
}

impl DateSelection {
    /// Converts the `--date` and `--yesterday` flags clap parsed into the day
    /// they stand for.
    #[must_use]
    pub const fn from_date_flags(date: Option<NaiveDate>, use_yesterday: bool) -> Self {
        if use_yesterday {
            return Self::Yesterday;
        }

        if let Some(date) = date {
            return Self::On(date);
        }

        Self::Unspecified
    }
}

/// Which week a summary was asked to cover. clap rejects `--week-of` together
/// with `--last-week`, so these stay mutually exclusive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeekSelection {
    /// Neither flag was passed, so the week containing today is used.
    Current,
    LastWeek,
    WeekOf(NaiveDate),
}

impl WeekSelection {
    /// Converts the `--week-of` and `--last-week` flags clap parsed into the
    /// week they stand for.
    #[must_use]
    pub const fn from_week_flags(monday_date: Option<NaiveDate>, use_last_week: bool) -> Self {
        if use_last_week {
            return Self::LastWeek;
        }

        if let Some(monday_date) = monday_date {
            return Self::WeekOf(monday_date);
        }

        Self::Current
    }
}

/// The date a command should act on. `None` means the caller applies its own
/// default.
pub fn resolve_selected_date(
    date_selection: DateSelection,
    today: NaiveDate,
) -> Result<Option<NaiveDate>> {
    match date_selection {
        DateSelection::Unspecified => Ok(None),

        DateSelection::On(date) => Ok(Some(date)),

        DateSelection::Yesterday => {
            let yesterday =
                today.pred_opt().ok_or_else(|| miette!("failed to determine yesterday's date"))?;

            Ok(Some(yesterday))
        }
    }
}

/// The Monday a weekly summary should cover. Unlike [`resolve_selected_date`]
/// this always produces a date, because the week command falls back to the week
/// containing today.
pub fn resolve_monday_date(week_selection: WeekSelection, today: NaiveDate) -> Result<NaiveDate> {
    match week_selection {
        WeekSelection::WeekOf(monday_date) => Ok(monday_date),

        WeekSelection::Current => get_monday_of_current_week(today),

        WeekSelection::LastWeek => get_monday_of_current_week(today)?
            .checked_sub_signed(Duration::days(DAYS_IN_WEEK))
            .ok_or_else(|| miette!("failed to determine the Monday of the previous week")),
    }
}

/// The Monday that begins the week containing `today`. On a Monday this is
/// today itself, so a week you only worked one day still summarizes.
fn get_monday_of_current_week(today: NaiveDate) -> Result<NaiveDate> {
    let days_since_monday = i64::from(today.weekday().num_days_from_monday());

    today
        .checked_sub_signed(Duration::days(days_since_monday))
        .ok_or_else(|| miette!("failed to determine the Monday of the current week"))
}

#[cfg(test)]
mod tests;
