use linear::LinearIssueTimestamps;

use super::*;

fn issue(
    identifier: &str,
    title: &str,
    status_name: &str,
    status_type: LinearStatusType,
) -> LinearIssue {
    LinearIssue {
        identifier: identifier.to_string(),
        title: title.to_string(),
        url: format!("https://linear.app/issue/{identifier}"),
        status_name: status_name.to_string(),
        status_type,
        timestamps: LinearIssueTimestamps::default(),
    }
}

#[test]
fn format_linear_issues_groups_and_orders_active_statuses() {
    let issues = vec![
        issue("ENG-1", "Todo issue", "Todo", LinearStatusType::Unstarted),
        issue("ENG-2", "Started issue", "In Progress", LinearStatusType::Started),
        issue("ENG-3", "Backlog issue", "Backlog", LinearStatusType::Backlog),
    ];

    let markdown = format_linear_issues(&issues);

    assert_eq!(
        markdown,
        "### In Progress\n- [ENG-2](https://linear.app/issue/ENG-2) Started issue\n\n### Todo\n- [ENG-1](https://linear.app/issue/ENG-1) Todo issue\n\n### Backlog\n- [ENG-3](https://linear.app/issue/ENG-3) Backlog issue"
    );
}

#[test]
fn format_linear_issues_groups_issues_sharing_a_status() {
    let issues = vec![
        issue("ENG-1", "First", "In Progress", LinearStatusType::Started),
        issue("ENG-2", "Second", "In Progress", LinearStatusType::Started),
    ];

    let markdown = format_linear_issues(&issues);

    assert_eq!(
        markdown,
        "### In Progress\n- [ENG-1](https://linear.app/issue/ENG-1) First\n- [ENG-2](https://linear.app/issue/ENG-2) Second"
    );
}

#[test]
fn format_linear_issues_collapses_whitespace_and_escapes_link_text() {
    let issues =
        vec![issue("ENG-1", "Fix [OAuth]\ncallback", "In Progress", LinearStatusType::Started)];

    let markdown = format_linear_issues(&issues);

    assert!(markdown.contains("Fix \\[OAuth\\] callback"));
}
