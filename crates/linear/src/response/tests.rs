use rmcp::model::{
    CallToolResult,
    ContentBlock,
};

use super::*;
use crate::LinearStatusType;

fn structured_result(value: Value) -> CallToolResult {
    CallToolResult::structured(value)
}

#[test]
fn parse_issue_page_reads_linear_mcp_issue_shape() {
    let result = structured_result(serde_json::json!({
        "issues": [{
            "id": "ISWF-3270",
            "title": "Remove organization:incidents flag",
            "url": "https://linear.app/getsentry/issue/ISWF-3270/remove-organizationincidents-flag",
            "status": "In Review",
            "statusType": "started"
        }]
    }));

    let page = parse_issue_page(result).expect("Linear MCP issue page should parse");
    let issue = page.issues.first().expect("page should contain one issue");

    assert_eq!(issue.identifier, "ISWF-3270");
    assert_eq!(issue.title, "Remove organization:incidents flag");
    assert_eq!(issue.status_name, "In Review");
    assert_eq!(issue.status_type, LinearStatusType::Started);
}

#[test]
fn parse_issue_page_reads_unrecognized_status_type_as_other() {
    let result = structured_result(serde_json::json!({
        "issues": [{
            "id": "ENG-1",
            "title": "Investigate the outage",
            "url": "https://linear.app/acme/issue/ENG-1",
            "status": "Triage",
            "statusType": "triage"
        }]
    }));

    let page = parse_issue_page(result).expect("unknown status types should parse");
    let issue = page.issues.first().expect("page should contain one issue");

    assert_eq!(issue.status_type, LinearStatusType::Other);
}

#[test]
fn take_next_cursor_returns_cursor_when_more_pages_remain() {
    let result = structured_result(serde_json::json!({
        "issues": [],
        "hasNextPage": true,
        "nextCursor": "next-page"
    }));

    let mut page = parse_issue_page(result).expect("issue page should parse");

    assert_eq!(page.take_next_cursor().as_deref(), Some("next-page"));
}

#[test]
fn take_next_cursor_stops_on_the_last_page() {
    let result = structured_result(serde_json::json!({
        "issues": [],
        "hasNextPage": false,
        "nextCursor": "stale-cursor"
    }));

    let mut page = parse_issue_page(result).expect("issue page should parse");

    assert_eq!(page.take_next_cursor(), None);
}

#[test]
fn parse_issue_page_reads_the_empty_text_response() {
    let result = CallToolResult::success(vec![ContentBlock::text("No issues found")]);

    let page = parse_issue_page(result).expect("empty text response should parse");

    assert_eq!(page.issues, []);
}

#[test]
fn parse_issue_page_reports_tool_errors() {
    let result = CallToolResult::error(vec![ContentBlock::text("rate limited")]);

    let error = parse_issue_page(result).expect_err("tool errors should fail");

    assert!(error.to_string().contains("rate limited"));
}
