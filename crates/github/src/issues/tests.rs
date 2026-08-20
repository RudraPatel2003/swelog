use chrono::NaiveDate;

use super::{
    get_merged_prs_search_query,
    get_opened_prs_search_query,
    parse_search_issues_response_text,
};

fn test_activity_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 7, 4).expect("test date should be valid")
}

#[test]
fn opened_prs_search_query_filters_to_activity_date() {
    let search_query = get_opened_prs_search_query("octocat", test_activity_date());

    assert_eq!(search_query, "author:octocat is:pr created:2026-07-04");
}

#[test]
fn merged_prs_search_query_filters_to_activity_date() {
    let search_query = get_merged_prs_search_query("octocat", test_activity_date());

    assert_eq!(search_query, "author:octocat is:pr merged:2026-07-04");
}

#[test]
fn parse_search_issues_response_text_extracts_issues() {
    let response_body = r#"
            {
              "total_count": 2,
              "incomplete_results": false,
              "items": [
                {
                  "title": "Add daily summary command",
                  "number": 42,
                  "repository_url": "https://api.github.com/repos/example/swelog",
                  "pull_request": {
                    "html_url": "https://github.com/example/swelog/pull/42"
                  }
                },
                {
                  "title": "Fix work log formatting",
                  "number": 43,
                  "repository_url": "https://api.github.com/repos/example/swelog",
                  "pull_request": {
                    "html_url": "https://github.com/example/swelog/pull/43"
                  }
                }
              ]
            }
        "#;

    let issues = parse_search_issues_response_text(response_body)
        .expect("GitHub search issues response should parse");

    assert_eq!(issues.len(), 2);
    assert_eq!(issues[0].title, "Add daily summary command");
    assert_eq!(issues[0].number, 42);
    assert_eq!(issues[0].repository_url, "https://api.github.com/repos/example/swelog");
    assert_eq!(issues[0].pull_request.html_url, "https://github.com/example/swelog/pull/42");
    assert_eq!(issues[1].title, "Fix work log formatting");
    assert_eq!(issues[1].number, 43);
    assert_eq!(issues[1].repository_url, "https://api.github.com/repos/example/swelog");
    assert_eq!(issues[1].pull_request.html_url, "https://github.com/example/swelog/pull/43");
}

#[test]
fn parse_search_issues_response_text_extracts_empty_issues() {
    let response_body = r#"
            {
              "total_count": 0,
              "incomplete_results": false,
              "items": []
            }
        "#;

    let issues = parse_search_issues_response_text(response_body)
        .expect("empty GitHub search issues response should parse");

    assert!(issues.is_empty());
}

#[test]
fn parse_search_issues_response_text_fails_when_required_field_is_missing() {
    let response_body = r#"
            {
              "items": [
                {
                  "title": "Add daily summary command",
                  "number": 42
                }
              ]
            }
        "#;

    parse_search_issues_response_text(response_body)
        .expect_err("missing pull_request field should fail");
}
