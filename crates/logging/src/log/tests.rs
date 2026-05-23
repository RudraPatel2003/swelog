use std::{
    fs,
    path::PathBuf,
};

use async_trait::async_trait;
use chrono::NaiveDate;
use config::{
    errors::SwelogFileNotFound,
    setup::DEFAULT_WORK_FILE_CONTENT,
    swelog_config::{
        LanguageModelProvider,
        SwelogConfig,
    },
};
use llm::LanguageModel;
use miette::Result;
use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const CONTEXT_FILE_CONTENT: &str = "backend engineer on platform team";
const WORK_FILE_CONTENT: &str = "debugged API timeout and reviewed auth PR";
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
    fn swelog_directory(&self) -> PathBuf {
        self.config.obsidian_vault_path.join(&self.config.swelog_folder_name)
    }

    fn context_file(&self) -> PathBuf {
        self.swelog_directory().join("context.md")
    }

    fn work_file(&self) -> PathBuf {
        self.swelog_directory().join("work.md")
    }

    fn daily_log_directory(&self) -> PathBuf {
        self.swelog_directory().join(&self.config.daily_log_folder_name)
    }

    fn daily_log_file(&self) -> PathBuf {
        let date = test_log_date();

        let formatted_date = date.format("%m-%d-%Y").to_string();

        self.daily_log_directory().join(format!("{formatted_date}.md"))
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
        swelog_folder_name: String::from("swelog"),
        daily_log_folder_name: String::from("daily"),
        weekly_log_folder_name: String::from("weekly"),
        language_model_provider: LanguageModelProvider::Ollama,
        ollama_model: String::from("llama3.2"),
    };

    TestContext { temporary_directory, config }
}

fn test_log_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 5, 23).expect("test date should be valid")
}

#[tokio::test]
async fn log_daily_work_writes_generated_daily_log() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    log_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        test_log_date(),
        false,
        false,
    )
    .await
    .expect("daily log should be written");

    let daily_log_content =
        fs::read_to_string(test_context.daily_log_file()).expect("daily log should be readable");

    assert!(daily_log_content.contains(WORK_FILE_CONTENT));
    assert!(daily_log_content.contains(CONTEXT_FILE_CONTENT));

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn log_daily_work_fails_when_context_file_is_missing() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::remove_file(test_context.context_file()).expect("context file should be removed");

    let error = log_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        test_log_date(),
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
async fn log_daily_work_fails_when_work_file_is_missing() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::remove_file(test_context.work_file()).expect("work file should be removed");

    let error = log_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        test_log_date(),
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
async fn log_daily_work_fails_when_daily_log_directory_is_missing() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::remove_dir(test_context.daily_log_directory())
        .expect("daily log directory should be removed");

    let error = log_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        test_log_date(),
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
async fn log_daily_work_fails_when_daily_log_exists_without_force() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::write(test_context.daily_log_file(), EXISTING_DAILY_LOG_CONTENT)
        .expect("existing daily log should be written");

    let error = log_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        test_log_date(),
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
async fn log_daily_work_overwrites_existing_daily_log_with_force() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    fs::write(test_context.daily_log_file(), EXISTING_DAILY_LOG_CONTENT)
        .expect("existing daily log should be written");

    log_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        test_log_date(),
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
async fn log_daily_work_resets_work_file_by_default() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    log_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        test_log_date(),
        false,
        false,
    )
    .await
    .expect("daily log should be written");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_content, DEFAULT_WORK_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[tokio::test]
async fn log_daily_work_keeps_work_file_when_keep_is_set() {
    let test_context = get_test_context();

    test_context.write_swelog_files();

    log_daily_work_from_config(
        &test_context.config,
        &FakeLanguageModel,
        test_log_date(),
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
