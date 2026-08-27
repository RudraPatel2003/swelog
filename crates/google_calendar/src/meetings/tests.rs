use super::*;
use crate::client::structs::CalendarEventPage;

fn only_meeting(response_text: &str) -> Meeting {
    parse_meetings(response_text).into_iter().next().expect("a meeting should be collected")
}

fn parse_meetings(response_text: &str) -> Vec<Meeting> {
    let event_page: CalendarEventPage =
        serde_json::from_str(response_text).expect("response should parse");

    collect_meetings(&event_page.items)
}

#[test]
fn collect_meetings_keeps_a_timed_event() {
    let response_text = r#"{
        "items": [
            {
                "summary": "Standup",
                "status": "confirmed",
                "start": { "dateTime": "2026-08-17T10:00:00-05:00" },
                "end": { "dateTime": "2026-08-17T10:15:00-05:00" }
            }
        ]
    }"#;

    assert_eq!(only_meeting(response_text).title, "Standup");
}

#[test]
fn collect_meetings_drops_all_day_events() {
    let response_text = r#"{
        "items": [
            {
                "summary": "Rudra OOO",
                "status": "confirmed",
                "start": { "date": "2026-08-17" },
                "end": { "date": "2026-08-18" }
            }
        ]
    }"#;

    assert_eq!(parse_meetings(response_text), Vec::new());
}

#[test]
fn collect_meetings_marks_cancelled_events_as_cancelled() {
    let response_text = r#"{
        "items": [
            {
                "summary": "Cancelled 1:1",
                "status": "cancelled",
                "start": { "dateTime": "2026-08-17T09:00:00-05:00" },
                "end": { "dateTime": "2026-08-17T09:30:00-05:00" }
            }
        ]
    }"#;

    assert_eq!(only_meeting(response_text).status, MeetingStatus::Cancelled);
}

#[test]
fn collect_meetings_marks_events_the_authorized_account_declined_as_declined() {
    let response_text = r#"{
        "items": [
            {
                "summary": "Eng all-hands",
                "status": "confirmed",
                "start": { "dateTime": "2026-08-17T11:00:00-05:00" },
                "end": { "dateTime": "2026-08-17T11:30:00-05:00" },
                "attendees": [
                    { "self": true, "responseStatus": "declined" },
                    { "responseStatus": "accepted" }
                ]
            }
        ]
    }"#;

    assert_eq!(only_meeting(response_text).status, MeetingStatus::Declined);
}

#[test]
fn collect_meetings_leaves_events_only_another_attendee_declined_as_scheduled() {
    let response_text = r#"{
        "items": [
            {
                "summary": "Design review",
                "status": "confirmed",
                "start": { "dateTime": "2026-08-17T13:00:00-05:00" },
                "end": { "dateTime": "2026-08-17T14:00:00-05:00" },
                "attendees": [
                    { "self": true, "responseStatus": "accepted" },
                    { "responseStatus": "declined" }
                ]
            }
        ]
    }"#;

    assert_eq!(only_meeting(response_text).status, MeetingStatus::Scheduled);
}

#[test]
fn collect_meetings_leaves_solo_events_as_scheduled() {
    let response_text = r#"{
        "items": [
            {
                "summary": "Focus block",
                "status": "confirmed",
                "start": { "dateTime": "2026-08-17T15:00:00-05:00" },
                "end": { "dateTime": "2026-08-17T16:00:00-05:00" }
            }
        ]
    }"#;

    assert_eq!(only_meeting(response_text).status, MeetingStatus::Scheduled);
}

#[test]
fn collect_meetings_names_an_untitled_event() {
    let response_text = r#"{
        "items": [
            {
                "status": "confirmed",
                "start": { "dateTime": "2026-08-17T15:00:00-05:00" },
                "end": { "dateTime": "2026-08-17T16:00:00-05:00" }
            }
        ]
    }"#;

    assert_eq!(only_meeting(response_text).title, "(No title)");
}

#[test]
fn collect_meetings_collapses_multiple_line_titles() {
    let response_text = r#"{
        "items": [
            {
                "summary": "Design\n  review   sync",
                "status": "confirmed",
                "start": { "dateTime": "2026-08-17T13:00:00-05:00" },
                "end": { "dateTime": "2026-08-17T14:00:00-05:00" }
            }
        ]
    }"#;

    assert_eq!(only_meeting(response_text).title, "Design review sync");
}
