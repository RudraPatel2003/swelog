use chrono::{
    NaiveDate,
    TimeZone,
};
use google_calendar::client::structs::MeetingStatus;

use super::*;

fn meeting(title: &str, start_hour: u32, end_hour: u32, status: MeetingStatus) -> Meeting {
    Meeting {
        title: title.to_string(),
        start: local_date_time(start_hour),
        end: local_date_time(end_hour),
        status,
    }
}

fn local_date_time(hour: u32) -> DateTime<Local> {
    let naive_date_time = NaiveDate::from_ymd_opt(2026, 8, 17)
        .expect("date should be valid")
        .and_hms_opt(hour, 0, 0)
        .expect("time should be valid");

    Local
        .from_local_datetime(&naive_date_time)
        .earliest()
        .expect("local date time should be unambiguous")
}

#[test]
fn format_meetings_separates_the_time_range_from_the_title_with_a_pipe() {
    let meetings = vec![meeting("Standup", 10, 11, MeetingStatus::Scheduled)];

    assert_eq!(format_meetings(&meetings), "- 10:00 AM - 11:00 AM | Standup");
}

#[test]
fn format_meetings_strikes_through_a_declined_meeting() {
    let meetings = vec![meeting("Eng all-hands", 11, 12, MeetingStatus::Declined)];

    assert_eq!(format_meetings(&meetings), "- ~~11:00 AM - 12:00 PM | Eng all-hands~~");
}

#[test]
fn format_meetings_strikes_through_a_cancelled_meeting() {
    let meetings = vec![meeting("Cancelled 1:1", 9, 10, MeetingStatus::Cancelled)];

    assert_eq!(format_meetings(&meetings), "- ~~9:00 AM - 10:00 AM | Cancelled 1:1~~");
}

#[test]
fn format_meetings_orders_meetings_by_start_time() {
    let meetings = vec![
        meeting("Design review", 13, 14, MeetingStatus::Scheduled),
        meeting("Standup", 10, 11, MeetingStatus::Scheduled),
    ];

    assert_eq!(
        format_meetings(&meetings),
        "- 10:00 AM - 11:00 AM | Standup\n- 1:00 PM - 2:00 PM | Design review"
    );
}

#[test]
fn format_meetings_keeps_a_struck_through_meeting_in_its_time_slot() {
    let meetings = vec![
        meeting("Design review", 13, 14, MeetingStatus::Scheduled),
        meeting("Eng all-hands", 11, 12, MeetingStatus::Declined),
        meeting("Standup", 10, 11, MeetingStatus::Scheduled),
    ];

    assert_eq!(
        format_meetings(&meetings),
        "- 10:00 AM - 11:00 AM | Standup\n- ~~11:00 AM - 12:00 PM | Eng all-hands~~\n- 1:00 PM - 2:00 PM | Design review"
    );
}

#[test]
fn format_meetings_writes_nothing_when_there_are_no_meetings() {
    assert_eq!(format_meetings(&[]), "");
}
