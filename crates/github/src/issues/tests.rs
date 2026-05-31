use super::parse_search_issues_response_text;

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
                  "pull_request": {
                    "html_url": "https://github.com/example/swelog/pull/42"
                  }
                },
                {
                  "title": "Fix work log formatting",
                  "number": 43,
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
    assert_eq!(issues[0].pull_request.html_url, "https://github.com/example/swelog/pull/42");
    assert_eq!(issues[1].title, "Fix work log formatting");
    assert_eq!(issues[1].number, 43);
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
