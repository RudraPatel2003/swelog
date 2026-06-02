use std::{
    fs,
    path::PathBuf,
};

use async_trait::async_trait;
use chrono::NaiveDate;
use config::{
    errors::SwelogFileNotFound,
    setup::{
        default_files::DEFAULT_WORK_FILE_CONTENT,
        swelog_paths::SwelogPaths,
    },
    swelog_config::SwelogConfig,
};
use llm::language_model::LanguageModel;
use miette::Result;
use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const CONTEXT_FILE_CONTENT: &str = "backend engineer on platform team";

const WORK_FILE_CONTENT: &str =
    "# Today's Work\n\n## Focus\n- Debug API timeout\n\n## Log\n- Reviewed auth PR\n";

const DEMOTED_WORK_FILE_CONTENT: &str =
    "### Today's Work\n\n#### Focus\n- Debug API timeout\n\n#### Log\n- Reviewed auth PR";
const EXISTING_DAILY_LOG_CONTENT: &str = "existing daily log";

struct TestContext {
    temporary_directory: TempDir,
    config: SwelogConfig,
}

struct FakeLanguageModel;

#[async_trait]
impl LanguageModel for FakeLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        Ok(format!("generated from prompt:\n{prompt}"))
    }
}

impl TestContext {
    fn swelog_paths(&self) -> SwelogPaths {
        SwelogPaths::new(&self.config)
    }

    fn context_file(&self) -> PathBuf {
        self.swelog_paths().context_file
    }

    fn work_file(&self) -> PathBuf {
        self.swelog_paths().work_file
    }

    fn daily_log_directory(&self) -> PathBuf {
        self.swelog_paths().daily_log_directory
    }

    fn daily_log_file(&self) -> PathBuf {
        let date = test_log_date();

        let daily_log_file_name = get_daily_log_file_name(&date);

        self.daily_log_directory().join(daily_log_file_name)
    }

    fn write_swelog_files(&self) {
        fs::create_dir_all(self.daily_log_directory())
            .expect("daily log directory should be created");

        fs::write(self.context_file(), CONTEXT_FILE_CONTENT)
            .expect("context file should be written");

        fs::write(self.work_file(), WORK_FILE_CONTENT).expect("work file should be written");
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

fn test_log_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 5, 23).expect("test date should be valid")
}

#[tokio::test]
async fn summarize_daily_work_writes_generated_daily_log() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    summarize_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &test_log_date(),
        false,
        false,
    )
    .await
    .expect("daily log should be written");

    let daily_log_content =
        fs::read_to_string(test_context.daily_log_file()).expect("daily log should be readable");

    assert!(daily_log_content.starts_with("generated from prompt:\n"));
    assert!(daily_log_content.contains(WORK_FILE_CONTENT));
    assert!(daily_log_content.contains(CONTEXT_FILE_CONTENT));
    assert!(
        daily_log_content.contains(&format!("## Original Notes\n\n{DEMOTED_WORK_FILE_CONTENT}\n"))
    );

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_daily_work_fails_when_context_file_is_missing() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::remove_file(test_context.context_file()).expect("context file should be removed");

    let error = summarize_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &test_log_date(),
        false,
        false,
    )
    .await
    .expect_err("missing context file should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.context_file());

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_daily_work_fails_when_work_file_is_missing() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::remove_file(test_context.work_file()).expect("work file should be removed");

    let error = summarize_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &test_log_date(),
        false,
        false,
    )
    .await
    .expect_err("missing work file should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.work_file());

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_daily_work_fails_when_daily_log_directory_is_missing() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::remove_dir(test_context.daily_log_directory())
        .expect("daily log directory should be removed");

    let error = summarize_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &test_log_date(),
        false,
        false,
    )
    .await
    .expect_err("missing daily log directory should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.daily_log_directory());

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_daily_work_fails_when_daily_log_exists_without_force() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::write(test_context.daily_log_file(), EXISTING_DAILY_LOG_CONTENT)
        .expect("existing daily log should be written");

    let error = summarize_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &test_log_date(),
        false,
        false,
    )
    .await
    .expect_err("existing daily log should fail without force");

    let error = error
        .downcast_ref::<DailyLogAlreadyExists>()
        .expect("error should be DailyLogAlreadyExists");

    assert_eq!(error.daily_log_file, test_context.daily_log_file());

    let daily_log_content =
        fs::read_to_string(test_context.daily_log_file()).expect("daily log should be readable");

    assert_eq!(daily_log_content, EXISTING_DAILY_LOG_CONTENT);

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_daily_work_overwrites_existing_daily_log_with_force() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::write(test_context.daily_log_file(), EXISTING_DAILY_LOG_CONTENT)
        .expect("existing daily log should be written");

    summarize_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &test_log_date(),
        true,
        false,
    )
    .await
    .expect("existing daily log should be overwritten with force");

    let daily_log_content =
        fs::read_to_string(test_context.daily_log_file()).expect("daily log should be readable");

    assert_ne!(daily_log_content, EXISTING_DAILY_LOG_CONTENT);
    assert!(daily_log_content.contains(WORK_FILE_CONTENT));

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_daily_work_resets_work_file_by_default() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    summarize_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &test_log_date(),
        false,
        false,
    )
    .await
    .expect("daily log should be written");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_content, DEFAULT_WORK_FILE_CONTENT);

    let daily_log_content =
        fs::read_to_string(test_context.daily_log_file()).expect("daily log should be readable");

    assert!(daily_log_content.contains("## Original Notes"));
    assert!(daily_log_content.contains(DEMOTED_WORK_FILE_CONTENT));

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn summarize_daily_work_keeps_work_file_when_keep_is_set() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    summarize_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        &test_log_date(),
        false,
        true,
    )
    .await
    .expect("daily log should be written");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_content, WORK_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn demote_markdown_headings_demotes_headings_by_two_levels() {
    let markdown = "# Today\n\n## Focus\n### Details\n- Reviewed auth PR\n";

    let demoted_markdown = demote_markdown_headings(markdown);

    assert_eq!(demoted_markdown, "### Today\n\n#### Focus\n##### Details\n- Reviewed auth PR\n");
}
