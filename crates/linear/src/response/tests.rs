use rmcp::model::CallToolResult;

use super::*;

fn structured_result(value: Value) -> CallToolResult {
    CallToolResult::structured(value)
}

#[test]
fn parse_issue_page_reads_structured_issues_and_cursor() {
    let result = structured_result(serde_json::json!({
        "issues": [{
            "identifier": "ENG-123",
            "title": "Ship feature",
            "url": "https://linear.app/issue/ENG-123",
            "status": { "id": "status-1", "name": "In Progress", "type": "started" }
        }],
        "hasNextPage": true,
        "nextCursor": "next-page"
    }));

    let page = parse_issue_page(result).expect("issue page should parse");

    assert_eq!(page.issues.len(), 1);
    assert_eq!(page.issues[0].identifier, "ENG-123");
    assert_eq!(page.next_cursor.as_deref(), Some("next-page"));
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
    let issue = &page.issues[0];

    assert_eq!(issue.identifier, "ISWF-3270");
    assert_eq!(issue.status_name, "In Review");
    assert_eq!(issue.status_type, Some(LinearStatusType::Started));
}

#[test]
fn parse_issue_page_uses_identifier_from_url_when_id_is_not_an_issue_identifier() {
    let result = structured_result(serde_json::json!({
        "issues": [{
            "id": "internal-uuid",
            "title": "Ship feature",
            "url": "https://linear.app/acme/issue/ENG-123/ship-feature",
            "status": "Todo",
            "statusType": "unstarted"
        }]
    }));

    let page = parse_issue_page(result).expect("issue identifier should parse from URL");

    assert_eq!(page.issues[0].identifier, "ENG-123");
}

#[test]
fn parse_issue_page_reads_identifier_from_issue_url_when_field_is_missing() {
    let result = structured_result(serde_json::json!({
        "issues": [{
            "id": "5c7dfe52-ae77-4f4d-b14f-ceeb7c43b675",
            "title": "Ship feature",
            "url": "https://linear.app/issue/ENG-123/ship-feature",
            "status": "In Progress"
        }],
        "hasNextPage": false
    }));

    let page = parse_issue_page(result).expect("issue page should parse");

    assert_eq!(page.issues[0].identifier, "ENG-123");
}

#[test]
fn parse_issue_page_reads_identifier_from_human_readable_id() {
    let result = structured_result(serde_json::json!({
        "issues": [{
            "id": "ENG-123",
            "title": "Ship feature",
            "url": "https://linear.app/acme/issue/ENG-123/ship-feature",
            "status": "In Progress"
        }],
        "hasNextPage": false
    }));

    let page = parse_issue_page(result).expect("issue page should parse");

    assert_eq!(page.issues[0].identifier, "ENG-123");
}

#[test]
fn resolve_issue_uses_status_catalog_when_type_is_missing() {
    let issue = UnresolvedLinearIssue {
        identifier: "ENG-123".to_string(),
        title: "Ship feature".to_string(),
        url: "https://linear.app/issue/ENG-123".to_string(),
        status_id: Some("status-1".to_string()),
        status_name: "In Progress".to_string(),
        status_type: None,
    };
    let statuses = HashMap::from([("status-1".to_string(), LinearStatusType::Started)]);

    let issue = resolve_issue(issue, &statuses).expect("status should resolve");

    assert_eq!(issue.status_type, LinearStatusType::Started);
}

#[test]
fn completed_and_canceled_statuses_are_inactive() {
    assert!(!LinearStatusType::Completed.is_active());
    assert!(!LinearStatusType::Canceled.is_active());
    assert!(LinearStatusType::Started.is_active());
}
