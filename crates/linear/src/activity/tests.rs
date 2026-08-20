use chrono::{
    Local,
    NaiveDate,
    TimeZone,
};

use super::*;
use crate::{
    LinearIssueTimestamps,
    LinearStatusType,
};

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("test date should be valid")
}

/// Builds a UTC instant from a local wall clock time so the assertions do not
/// depend on the time zone the tests run in.
fn local_instant(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    Local
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .single()
        .expect("test instant should be unambiguous")
        .to_utc()
}

fn issue(timestamps: LinearIssueTimestamps) -> LinearIssue {
    LinearIssue {
        identifier: "ENG-1".to_string(),
        title: "Investigate the outage".to_string(),
        url: "https://linear.app/acme/issue/ENG-1".to_string(),
        status_name: "In Progress".to_string(),
        status_type: LinearStatusType::Started,
        timestamps,
    }
}

#[test]
fn get_updated_after_filter_starts_a_day_before_the_requested_date() {
    let updated_after =
        get_updated_after_filter(date(2026, 8, 17)).expect("filter should be built");

    assert_eq!(updated_after, "2026-08-16T00:00:00Z");
}

#[test]
fn get_updated_after_filter_crosses_a_year_boundary() {
    let updated_after = get_updated_after_filter(date(2026, 1, 1)).expect("filter should be built");

    assert_eq!(updated_after, "2025-12-31T00:00:00Z");
}

#[test]
fn was_issue_worked_on_matches_an_issue_updated_on_the_date() {
    let issue = issue(LinearIssueTimestamps {
        updated_at: Some(local_instant(2026, 8, 17, 14, 30)),
        ..LinearIssueTimestamps::default()
    });

    assert!(was_issue_worked_on(&issue, date(2026, 8, 17)));
}

#[test]
fn was_issue_worked_on_matches_an_issue_completed_on_the_date_but_updated_later() {
    let issue = issue(LinearIssueTimestamps {
        completed_at: Some(local_instant(2026, 8, 17, 16, 0)),
        updated_at: Some(local_instant(2026, 8, 19, 9, 0)),
        ..LinearIssueTimestamps::default()
    });

    assert!(was_issue_worked_on(&issue, date(2026, 8, 17)));
}

#[test]
fn was_issue_worked_on_ignores_an_issue_from_a_different_date() {
    let issue = issue(LinearIssueTimestamps {
        created_at: Some(local_instant(2026, 8, 10, 9, 0)),
        started_at: Some(local_instant(2026, 8, 11, 9, 0)),
        updated_at: Some(local_instant(2026, 8, 18, 9, 0)),
        ..LinearIssueTimestamps::default()
    });

    assert!(!was_issue_worked_on(&issue, date(2026, 8, 17)));
}

#[test]
fn was_issue_worked_on_ignores_an_issue_without_timestamps() {
    let issue = issue(LinearIssueTimestamps::default());

    assert!(!was_issue_worked_on(&issue, date(2026, 8, 17)));
}

#[test]
fn was_issue_worked_on_reads_timestamps_in_the_local_time_zone() {
    let issue = issue(LinearIssueTimestamps {
        updated_at: Some(local_instant(2026, 8, 17, 23, 30)),
        ..LinearIssueTimestamps::default()
    });

    assert!(was_issue_worked_on(&issue, date(2026, 8, 17)));
    assert!(!was_issue_worked_on(&issue, date(2026, 8, 18)));
}
