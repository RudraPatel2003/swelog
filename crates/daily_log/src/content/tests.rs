use super::*;

const WORK_FILE_CONTENT: &str =
    "# Today's Work\n\n## Focus\n- Debug API timeout\n\n## Log\n- Reviewed auth PR\n";

fn test_log_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 5, 23).expect("test date should be valid")
}

#[test]
fn build_daily_log_content_replaces_the_work_file_heading() {
    let daily_log_content = build_daily_log_content(WORK_FILE_CONTENT, &test_log_date());

    assert_eq!(
        daily_log_content,
        "# Daily Log - 05-23-2026\n\n## Focus\n- Debug API timeout\n\n## Log\n- Reviewed auth \
         PR\n"
    );
}

#[test]
fn build_daily_log_content_keeps_integration_sections_verbatim() {
    let work_file_content = "# Today's Work\n\n## GitHub\n- Merged [#412](https://example.com)\n\n## Log\n<!-- Quick capture. -->\n- Fixed flaky billing test\n";

    let daily_log_content = build_daily_log_content(work_file_content, &test_log_date());

    assert_eq!(
        daily_log_content,
        "# Daily Log - 05-23-2026\n\n## GitHub\n- Merged [#412](https://example.com)\n\n## \
         Log\n<!-- Quick capture. -->\n- Fixed flaky billing test\n"
    );
}

#[test]
fn build_daily_log_content_replaces_only_the_first_heading() {
    let work_file_content = "# Today's Work\n\n# Second Heading\n- Reviewed auth PR\n";

    let daily_log_content = build_daily_log_content(work_file_content, &test_log_date());

    assert_eq!(
        daily_log_content,
        "# Daily Log - 05-23-2026\n\n# Second Heading\n- Reviewed auth PR\n"
    );
}
