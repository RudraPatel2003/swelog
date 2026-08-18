use std::{
    fs,
    path::PathBuf,
};

use config::{
    errors::SwelogFileNotFound,
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
};
use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const WORK_ITEM: &str = "- Meeting with manager";

struct TestContext {
    temporary_directory: TempDir,
    config: SwelogConfig,
}

impl TestContext {
    fn swelog_paths(&self) -> SwelogPaths {
        SwelogPaths::new(&self.config)
    }

    fn swelog_directory(&self) -> PathBuf {
        self.swelog_paths().swelog_directory
    }

    fn work_file(&self) -> PathBuf {
        self.swelog_paths().work_file
    }

    fn write_work_file(&self, content: &str) {
        fs::create_dir_all(self.swelog_directory()).expect("swelog directory should be created");

        fs::write(self.work_file(), content).expect("work file should be written");
    }
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let config = SwelogConfig {
        obsidian_vault_path: temporary_directory.path().to_path_buf(),
        ..SwelogConfig::get_default_config()
    };

    TestContext { temporary_directory, config }
}

#[test]
fn log_work_item_appends_bullet_to_log_section() {
    let test_context = get_test_context();

    test_context.write_work_file(
        "# Today's Work\n\n## Focus\n- Finish report\n\n## Log\n<!-- Quick capture. -->\n\n## Follow-ups\n- Update ticket\n",
    );

    append_work_file_section_from_config(&test_context.config, WORK_ITEM, "Log")
        .expect("work item should be appended");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(
        work_file_content,
        "# Today's Work\n\n## Focus\n- Finish report\n\n## Log\n<!-- Quick capture. -->\n- Meeting with manager\n\n## Follow-ups\n- Update ticket\n"
    );

    drop(test_context.temporary_directory);
}

#[test]
fn log_work_item_inserts_newline_when_log_section_has_no_trailing_newline() {
    let test_context = get_test_context();

    test_context.write_work_file("# Today's Work\n\n## Log");

    append_work_file_section_from_config(&test_context.config, WORK_ITEM, "Log")
        .expect("work item should be appended");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_content, "# Today's Work\n\n## Log\n- Meeting with manager\n");

    drop(test_context.temporary_directory);
}

#[test]
fn overwrite_work_file_section_from_config_replaces_section_content() {
    let test_context = get_test_context();

    test_context.write_work_file(
        "# Today's Work\n\n## GitHub\n- Existing GitHub item\n\n## Log\n- Existing log item\n",
    );

    overwrite_work_file_section_from_config(&test_context.config, "- New GitHub item", "GitHub")
        .expect("GitHub section should be replaced");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(
        work_file_content,
        "# Today's Work\n\n## GitHub\n- New GitHub item\n\n## Log\n- Existing log item\n"
    );

    drop(test_context.temporary_directory);
}

#[test]
fn log_work_item_fails_when_work_file_is_missing() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.swelog_directory())
        .expect("swelog directory should be created");

    let error = append_work_file_section_from_config(&test_context.config, WORK_ITEM, "Log")
        .expect_err("missing work file should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.work_file());

    drop(test_context.temporary_directory);
}

#[test]
fn append_to_section_does_not_insert_blank_line_after_existing_content() {
    let markdown = "# Today's Work\n\n## Log\n- Existing item\n\n## Follow-ups\n- Update ticket\n";

    let updated_markdown = append_to_section(markdown, "Log", "- Meeting with manager")
        .expect("append to section should succeed");

    assert_eq!(
        updated_markdown,
        "# Today's Work\n\n## Log\n- Existing item\n- Meeting with manager\n\n## Follow-ups\n- Update ticket\n"
    );
}

#[test]
fn append_to_section_keeps_blank_line_before_next_section() {
    let markdown = "# Today's Work\n\n## Log\n\n## Follow-ups\n- Update ticket\n";

    let updated_markdown = append_to_section(markdown, "Log", "- Meeting with manager")
        .expect("append to section should succeed");

    assert_eq!(
        updated_markdown,
        "# Today's Work\n\n## Log\n- Meeting with manager\n\n## Follow-ups\n- Update ticket\n"
    );
}

#[test]
fn overwrite_section_replaces_existing_content() {
    let markdown = "# Today's Work\n\n## Log\n- Existing item\n\n## Follow-ups\n- Update ticket\n";

    let updated_markdown = overwrite_section(markdown, "Log", "- Meeting with manager")
        .expect("overwrite section should succeed");

    assert_eq!(
        updated_markdown,
        "# Today's Work\n\n## Log\n- Meeting with manager\n\n## Follow-ups\n- Update ticket\n"
    );
}

#[test]
fn overwrite_section_replaces_last_section() {
    let markdown = "# Today's Work\n\n## Log\n- Existing item\n";

    let updated_markdown = overwrite_section(markdown, "Log", "- Meeting with manager")
        .expect("overwrite section should succeed");

    assert_eq!(updated_markdown, "# Today's Work\n\n## Log\n- Meeting with manager\n");
}

#[test]
fn upsert_managed_section_inserts_before_log() {
    let markdown = "# Today's Work\n\n## Focus\n- Ship it\n\n## Log\n- Existing log\n";

    let updated_markdown = upsert_managed_section(
        markdown,
        "linear",
        "Linear",
        "### In Progress\n- [ENG-123](https://linear.app/issue/ENG-123) Ship it",
    )
    .expect("managed section should be inserted");

    assert_eq!(
        updated_markdown,
        "# Today's Work\n\n## Focus\n- Ship it\n\n<!-- swelog:managed linear:start -->\n## Linear\n### In Progress\n- [ENG-123](https://linear.app/issue/ENG-123) Ship it\n<!-- swelog:managed linear:end -->\n\n## Log\n- Existing log\n"
    );
}

#[test]
fn upsert_managed_section_replaces_existing_managed_block() {
    let markdown = "# Today's Work\n\n<!-- swelog:managed linear:start -->\n## Linear\n- Old\n<!-- swelog:managed linear:end -->\n\n## Log\n";

    let updated_markdown = upsert_managed_section(markdown, "linear", "Linear", "- New")
        .expect("managed section should be replaced");

    assert_eq!(
        updated_markdown,
        "# Today's Work\n\n<!-- swelog:managed linear:start -->\n## Linear\n- New\n<!-- swelog:managed linear:end -->\n\n## Log\n"
    );
}

#[test]
fn upsert_managed_section_adopts_exact_existing_heading() {
    let markdown = "# Today's Work\n\n## Linear\n- Manual placeholder\n\n## Log\n";

    let updated_markdown = upsert_managed_section(markdown, "linear", "Linear", "- New")
        .expect("existing section should be adopted");

    assert_eq!(
        updated_markdown,
        "# Today's Work\n\n<!-- swelog:managed linear:start -->\n## Linear\n- New\n<!-- swelog:managed linear:end -->\n\n## Log\n"
    );
}

#[test]
fn upsert_managed_section_ignores_heading_in_code_block() {
    let markdown = "# Today's Work\n\n```markdown\n## Linear\n```\n\n## Log\n";

    let updated_markdown = upsert_managed_section(markdown, "linear", "Linear", "- New")
        .expect("managed section should be inserted");

    assert!(updated_markdown.contains("```markdown\n## Linear\n```"));
    assert!(updated_markdown.contains("<!-- swelog:managed linear:start -->\n## Linear\n- New"));
}

#[test]
fn remove_managed_section_removes_only_marked_block() {
    let markdown = "# Today's Work\n\n<!-- swelog:managed linear:start -->\n## Linear\n- Issue\n<!-- swelog:managed linear:end -->\n\n## Log\n";

    let updated_markdown =
        remove_managed_section(markdown, "linear").expect("managed section should be removed");

    assert_eq!(updated_markdown, "# Today's Work\n\n## Log\n");
}

#[test]
fn remove_managed_section_preserves_unmarked_section() {
    let markdown = "# Today's Work\n\n## Linear\n- Manual\n\n## Log\n";

    let updated_markdown =
        remove_managed_section(markdown, "linear").expect("unmarked section should be preserved");

    assert_eq!(updated_markdown, markdown);
}
