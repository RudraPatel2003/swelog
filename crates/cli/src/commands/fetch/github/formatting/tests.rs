use github::issues::PullRequest;

use super::*;

fn issue(repository_url: &str, number: u64) -> Issue {
    Issue {
        title: format!("PR {number}"),
        number,
        pull_request: PullRequest {
            html_url: format!("https://github.com/swelog-cli/swelog-cli/pull/{number}"),
        },
        repository_url: repository_url.to_string(),
    }
}

const OPENED_AND_MERGED_SECTIONS: &str = r"Opened:
- [swelog-cli/swelog-cli#123](https://github.com/swelog-cli/swelog-cli/pull/123)

Merged:
- [swelog-cli/swelog-cli#789](https://github.com/swelog-cli/swelog-cli/pull/789)";

const OPENED_SECTION_ONLY: &str = r"Opened:
- [swelog-cli/swelog-cli#123](https://github.com/swelog-cli/swelog-cli/pull/123)";

const OPENED_SECTION_WITH_MULTIPLE_PULL_REQUESTS: &str = r"Opened:
- [swelog-cli/swelog-cli#123](https://github.com/swelog-cli/swelog-cli/pull/123)
- [swelog-cli/swelog-cli#456](https://github.com/swelog-cli/swelog-cli/pull/456)";

#[test]
fn format_github_activity_lists_opened_and_merged_sections() {
    let opened_prs = vec![issue("https://api.github.com/repos/swelog-cli/swelog-cli", 123)];
    let merged_prs = vec![issue("https://api.github.com/repos/swelog-cli/swelog-cli", 789)];

    let markdown = format_github_activity(&opened_prs, &merged_prs);

    assert_eq!(markdown, OPENED_AND_MERGED_SECTIONS);
}

#[test]
fn format_github_activity_omits_a_section_with_no_pull_requests() {
    let opened_prs = vec![issue("https://api.github.com/repos/swelog-cli/swelog-cli", 123)];

    let markdown = format_github_activity(&opened_prs, &[]);

    assert_eq!(markdown, OPENED_SECTION_ONLY);
}

#[test]
fn format_github_activity_lists_multiple_pull_requests_in_a_section() {
    let opened_prs = vec![
        issue("https://api.github.com/repos/swelog-cli/swelog-cli", 123),
        issue("https://api.github.com/repos/swelog-cli/swelog-cli", 456),
    ];

    let markdown = format_github_activity(&opened_prs, &[]);

    assert_eq!(markdown, OPENED_SECTION_WITH_MULTIPLE_PULL_REQUESTS);
}

#[test]
fn format_github_activity_is_empty_when_there_is_no_activity() {
    let markdown = format_github_activity(&[], &[]);

    assert_eq!(markdown, "");
}
