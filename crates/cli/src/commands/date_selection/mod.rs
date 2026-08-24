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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateSelection {
    Unspecified,
    Yesterday,
    On(NaiveDate),
}

impl DateSelection {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeekSelection {
    Current,
    LastWeek,
    WeekOf(NaiveDate),
}

impl WeekSelection {
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

pub fn resolve_monday_date(week_selection: WeekSelection, today: NaiveDate) -> Result<NaiveDate> {
    match week_selection {
        WeekSelection::WeekOf(monday_date) => Ok(monday_date),

        WeekSelection::Current => get_monday_of_current_week(today),

        WeekSelection::LastWeek => get_monday_of_current_week(today)?
            .checked_sub_signed(Duration::days(DAYS_IN_WEEK))
            .ok_or_else(|| miette!("failed to determine the Monday of the previous week")),
    }
}

fn get_monday_of_current_week(today: NaiveDate) -> Result<NaiveDate> {
    let days_since_monday = i64::from(today.weekday().num_days_from_monday());

    today
        .checked_sub_signed(Duration::days(days_since_monday))
        .ok_or_else(|| miette!("failed to determine the Monday of the current week"))
}

#[cfg(test)]
mod tests;
