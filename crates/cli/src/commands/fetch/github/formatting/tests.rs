use github::issues::PullRequest;

use super::*;

const SWELOG_REPOSITORY_URL: &str = "https://api.github.com/repos/swelog-cli/swelog-cli";

fn get_mock_issue(repository_url: &str, number: u64) -> Issue {
    Issue {
        title: format!("PR {number}"),
        number,
        pull_request: PullRequest {
            html_url: format!("https://github.com/swelog-cli/swelog-cli/pull/{number}"),
        },
        repository_url: repository_url.to_string(),
    }
}

const OPENED_AND_MERGED_SECTIONS: &str = r#"### Opened
- "PR 123" ([#123](https://github.com/swelog-cli/swelog-cli/pull/123)) in [swelog-cli/swelog-cli](https://github.com/swelog-cli/swelog-cli)

### Merged
- "PR 789" ([#789](https://github.com/swelog-cli/swelog-cli/pull/789)) in [swelog-cli/swelog-cli](https://github.com/swelog-cli/swelog-cli)"#;

#[test]
fn format_github_activity_lists_opened_and_merged_sections() {
    let opened_pr = get_mock_issue(SWELOG_REPOSITORY_URL, 123);

    let opened_prs = vec![opened_pr];

    let merged_pr = get_mock_issue(SWELOG_REPOSITORY_URL, 789);

    let merged_prs = vec![merged_pr];

    let markdown = format_github_activity(&opened_prs, &merged_prs);

    assert_eq!(markdown, OPENED_AND_MERGED_SECTIONS);
}

const OPENED_SECTION_ONLY: &str = r#"### Opened
- "PR 123" ([#123](https://github.com/swelog-cli/swelog-cli/pull/123)) in [swelog-cli/swelog-cli](https://github.com/swelog-cli/swelog-cli)"#;

#[test]
fn format_github_activity_omits_a_section_with_no_pull_requests() {
    let opened_pr = get_mock_issue(SWELOG_REPOSITORY_URL, 123);

    let opened_prs = vec![opened_pr];

    let markdown = format_github_activity(&opened_prs, &[]);

    assert_eq!(markdown, OPENED_SECTION_ONLY);
}

const OPENED_SECTION_WITH_MULTIPLE_PULL_REQUESTS: &str = r#"### Opened
- "PR 123" ([#123](https://github.com/swelog-cli/swelog-cli/pull/123)) in [swelog-cli/swelog-cli](https://github.com/swelog-cli/swelog-cli)
- "PR 456" ([#456](https://github.com/swelog-cli/swelog-cli/pull/456)) in [swelog-cli/swelog-cli](https://github.com/swelog-cli/swelog-cli)"#;

#[test]
fn format_github_activity_lists_multiple_pull_requests_in_a_section() {
    let first_opened_pr = get_mock_issue(SWELOG_REPOSITORY_URL, 123);

    let second_opened_pr = get_mock_issue(SWELOG_REPOSITORY_URL, 456);

    let opened_prs = vec![first_opened_pr, second_opened_pr];

    let markdown = format_github_activity(&opened_prs, &[]);

    assert_eq!(markdown, OPENED_SECTION_WITH_MULTIPLE_PULL_REQUESTS);
}

#[test]
fn format_github_activity_is_empty_when_there_is_no_activity() {
    let markdown = format_github_activity(&[], &[]);

    assert_eq!(markdown, "");
}

const PULL_REQUEST_IN_ANOTHER_REPOSITORY: &str = r#"### Opened
- "PR 37" ([#37](https://github.com/swelog-cli/swelog-cli/pull/37)) in [getsentry/sentry](https://github.com/getsentry/sentry)"#;

#[test]
fn format_github_activity_links_the_repository_the_pull_request_belongs_to() {
    let opened_pr = get_mock_issue("https://api.github.com/repos/getsentry/sentry", 37);

    let markdown = format_github_activity(&[opened_pr], &[]);

    assert_eq!(markdown, PULL_REQUEST_IN_ANOTHER_REPOSITORY);
}
