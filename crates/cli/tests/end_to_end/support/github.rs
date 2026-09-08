use httpmock::{
    Method::GET,
    Mock,
    MockServer,
};

use crate::support::sandbox::GITHUB_TOKEN;

pub const GITHUB_USERNAME: &str = "octocat";

const USER_RESPONSE: &str = r#"{ "login": "octocat", "id": 1 }"#;

const OPENED_PRS_RESPONSE: &str = r#"{
  "total_count": 1,
  "incomplete_results": false,
  "items": [
    {
      "title": "Add end-to-end tests",
      "number": 42,
      "repository_url": "https://api.github.com/repos/example/swelog",
      "pull_request": { "html_url": "https://github.com/example/swelog/pull/42" }
    }
  ]
}"#;

const MERGED_PRS_RESPONSE: &str = r#"{
  "total_count": 1,
  "incomplete_results": false,
  "items": [
    {
      "title": "Fix work file formatting",
      "number": 43,
      "repository_url": "https://api.github.com/repos/example/swelog",
      "pull_request": { "html_url": "https://github.com/example/swelog/pull/43" }
    }
  ]
}"#;

const NO_PRS_RESPONSE: &str = r#"{ "total_count": 0, "incomplete_results": false, "items": [] }"#;

pub const GITHUB_SECTION: &str = r#"## GitHub
### Opened
- "Add end-to-end tests" ([#42](https://github.com/example/swelog/pull/42)) in [example/swelog](https://github.com/example/swelog)

### Merged
- "Fix work file formatting" ([#43](https://github.com/example/swelog/pull/43)) in [example/swelog](https://github.com/example/swelog)"#;

pub struct GitHubMocks<'server> {
    pub user: Mock<'server>,
    pub opened_prs: Mock<'server>,
    pub merged_prs: Mock<'server>,
}

pub fn mock_github_with_activity_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> GitHubMocks<'server> {
    GitHubMocks {
        user: mock_user(server),
        opened_prs: mock_search(server, &opened_query(activity_date), OPENED_PRS_RESPONSE),
        merged_prs: mock_search(server, &merged_query(activity_date), MERGED_PRS_RESPONSE),
    }
}

pub fn mock_github_with_no_activity_on<'server>(
    server: &'server MockServer,
    activity_date: &str,
) -> GitHubMocks<'server> {
    GitHubMocks {
        user: mock_user(server),
        opened_prs: mock_search(server, &opened_query(activity_date), NO_PRS_RESPONSE),
        merged_prs: mock_search(server, &merged_query(activity_date), NO_PRS_RESPONSE),
    }
}

pub fn mock_github_rejecting_the_token(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(GET).path("/user");

        then.status(401)
            .header("content-type", "application/json")
            .body(r#"{ "message": "Bad credentials" }"#);
    })
}

fn mock_user(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(GET).path("/user").header("authorization", format!("Bearer {GITHUB_TOKEN}"));

        then.status(200).header("content-type", "application/json").body(USER_RESPONSE);
    })
}

fn mock_search<'server>(
    server: &'server MockServer,
    search_query: &str,
    response: &str,
) -> Mock<'server> {
    server.mock(|when, then| {
        when.method(GET)
            .path("/search/issues")
            .query_param("q", search_query)
            .header("authorization", format!("Bearer {GITHUB_TOKEN}"));

        then.status(200).header("content-type", "application/json").body(response);
    })
}

fn opened_query(activity_date: &str) -> String {
    format!("author:{GITHUB_USERNAME} is:pr created:{}", to_iso_date(activity_date))
}

fn merged_query(activity_date: &str) -> String {
    format!("author:{GITHUB_USERNAME} is:pr merged:{}", to_iso_date(activity_date))
}

fn to_iso_date(date: &str) -> String {
    crate::support::sandbox::parse_date(date).format("%Y-%m-%d").to_string()
}
