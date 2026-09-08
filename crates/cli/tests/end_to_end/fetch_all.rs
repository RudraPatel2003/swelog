use httpmock::MockServer;
use predicates::str::contains;

use crate::support::{
    github::{
        GITHUB_SECTION,
        mock_github_with_activity_on,
    },
    sandbox::{
        ACTIVITY_DATE,
        GITHUB_TOKEN,
        SwelogSandbox,
        WRITTEN_WORK_FILE_CONTENT,
    },
};

#[test]
fn fetch_all_runs_only_the_sources_with_a_credential() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox.write_work_file(WRITTEN_WORK_FILE_CONTENT);

    let github = MockServer::start();

    let github_mocks = mock_github_with_activity_on(&github, ACTIVITY_DATE);

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .env("SWELOG_GITHUB_API_URL", github.base_url())
        .args(["fetch", "all", "--date", ACTIVITY_DATE])
        .assert()
        .success()
        .stdout(contains("Running the fetch commands you have configured: GitHub."))
        .stdout(contains("Recorded 2 GitHub PRs in your work file."));

    github_mocks.user.assert();

    assert!(sandbox.read_work_file().contains(GITHUB_SECTION));
}

#[test]
fn fetch_all_fails_when_no_source_has_a_credential() {
    let sandbox = SwelogSandbox::new();

    sandbox.setup();

    sandbox
        .swelog()
        .args(["fetch", "all", "--date", ACTIVITY_DATE])
        .assert()
        .failure()
        .stderr(contains("no fetch commands are configured"))
        .stderr(contains("swelog fetch status"));
}

#[test]
fn fetch_status_explains_what_each_source_still_needs() {
    let sandbox = SwelogSandbox::new();

    sandbox
        .swelog()
        .env("GITHUB_TOKEN", GITHUB_TOKEN)
        .args(["fetch", "status"])
        .assert()
        .success()
        .stdout(contains("GitHub              included"))
        .stdout(contains("Linear              not included, Linear authorization is not stored"))
        .stdout(contains(
            "Google Calendar     not included, Google Calendar authorization is not stored",
        ));
}

#[test]
fn fetch_status_reports_missing_linear_configuration_once_authorized() {
    let sandbox = SwelogSandbox::new();

    sandbox.store_credential(credentials::credential::Credential::Linear, "{}");

    sandbox
        .swelog()
        .args(["fetch", "status"])
        .assert()
        .success()
        .stdout(contains("Linear              not included, linearUsername is not configured"));
}
