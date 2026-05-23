use std::{
    fs,
    path::PathBuf,
};

use tempfile::{
    TempDir,
    tempdir,
};

use super::*;
use crate::swelog_config::SupportedLlm;

const EXISTING_CONTEXT_FILE_CONTENT: &str = "existing context";
const EXISTING_WORK_FILE_CONTENT: &str = "existing work";

struct TestContext {
    temporary_directory: TempDir,
    config: SwelogConfig,
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

    fn weekly_log_directory(&self) -> PathBuf {
        self.swelog_directory().join(&self.config.weekly_log_folder_name)
    }
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let config = SwelogConfig {
        obsidian_vault_path: temporary_directory.path().to_path_buf(),
        swelog_folder_name: String::from("swelog"),
        daily_log_folder_name: String::from("daily"),
        weekly_log_folder_name: String::from("weekly"),
        llm: SupportedLlm::Ollama,
        ollama_model: String::from("llama3.2"),
    };

    TestContext { temporary_directory, config }
}

#[test]
fn setup_creates_swelog_files_and_directories() {
    let test_context = get_test_context();

    let overwrite_existing_files = false;

    setup_swelog_files_from_config(&test_context.config, overwrite_existing_files)
        .expect("swelog files should be created");

    let context_file_contents =
        fs::read_to_string(test_context.context_file()).expect("context file should be readable");

    let work_file_contents =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(context_file_contents, DEFAULT_CONTEXT_FILE_CONTENT);
    assert_eq!(work_file_contents, DEFAULT_WORK_FILE_CONTENT);
    assert!(test_context.daily_log_directory().is_dir());
    assert!(test_context.weekly_log_directory().is_dir());

    drop(test_context.temporary_directory);
}

#[test]
fn setup_uses_configured_folder_names() {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let config = SwelogConfig {
        obsidian_vault_path: temporary_directory.path().to_path_buf(),
        swelog_folder_name: String::from("accomplishments"),
        daily_log_folder_name: String::from("days"),
        weekly_log_folder_name: String::from("weeks"),
        llm: SupportedLlm::Ollama,
        ollama_model: String::from("llama3.2"),
    };

    let overwrite_existing_files = false;

    setup_swelog_files_from_config(&config, overwrite_existing_files)
        .expect("swelog files should be created");

    assert!(temporary_directory.path().join("accomplishments/context.md").is_file());
    assert!(temporary_directory.path().join("accomplishments/work.md").is_file());
    assert!(temporary_directory.path().join("accomplishments/days").is_dir());
    assert!(temporary_directory.path().join("accomplishments/weeks").is_dir());

    drop(temporary_directory);
}

#[test]
fn setup_fails_when_context_file_exists_without_force() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.swelog_directory())
        .expect("swelog directory should be created");

    fs::write(test_context.context_file(), EXISTING_CONTEXT_FILE_CONTENT)
        .expect("existing context file should be written");

    let overwrite_existing_files = false;

    let result = setup_swelog_files_from_config(&test_context.config, overwrite_existing_files);

    let error = result.expect_err("existing context file should not be overwritten without force");

    let error = error
        .downcast_ref::<SetupFilesAlreadyExist>()
        .expect("error should be SetupFilesAlreadyExist");

    assert_eq!(error.setup_path, test_context.context_file());

    let context_file_contents =
        fs::read_to_string(test_context.context_file()).expect("context file should be readable");

    assert_eq!(context_file_contents, EXISTING_CONTEXT_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn setup_fails_when_work_file_exists_without_force() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.swelog_directory())
        .expect("swelog directory should be created");
    fs::write(test_context.work_file(), EXISTING_WORK_FILE_CONTENT)
        .expect("existing work file should be written");

    let overwrite_existing_files = false;

    let result = setup_swelog_files_from_config(&test_context.config, overwrite_existing_files);

    let error = result.expect_err("existing work file should not be overwritten without force");

    let error = error
        .downcast_ref::<SetupFilesAlreadyExist>()
        .expect("error should be SetupFilesAlreadyExist");

    assert_eq!(error.setup_path, test_context.work_file());

    let work_file_contents =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_contents, EXISTING_WORK_FILE_CONTENT);

    drop(test_context.temporary_directory);
}

#[test]
fn setup_fails_when_daily_log_directory_exists_without_force() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.daily_log_directory())
        .expect("daily log directory should be created");

    let overwrite_existing_files = false;

    let result = setup_swelog_files_from_config(&test_context.config, overwrite_existing_files);

    let error =
        result.expect_err("existing daily log directory should not be overwritten without force");

    let error = error
        .downcast_ref::<SetupFilesAlreadyExist>()
        .expect("error should be SetupFilesAlreadyExist");

    assert_eq!(error.setup_path, test_context.daily_log_directory());

    drop(test_context.temporary_directory);
}

#[test]
fn setup_fails_when_weekly_log_directory_exists_without_force() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.weekly_log_directory())
        .expect("weekly log directory should be created");

    let overwrite_existing_files = false;

    let result = setup_swelog_files_from_config(&test_context.config, overwrite_existing_files);

    let error =
        result.expect_err("existing weekly log directory should not be overwritten without force");

    let error = error
        .downcast_ref::<SetupFilesAlreadyExist>()
        .expect("error should be SetupFilesAlreadyExist");

    assert_eq!(error.setup_path, test_context.weekly_log_directory());

    drop(test_context.temporary_directory);
}

#[test]
fn setup_overwrites_existing_files_when_force_is_set() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.swelog_directory())
        .expect("swelog directory should be created");

    fs::write(test_context.context_file(), EXISTING_CONTEXT_FILE_CONTENT)
        .expect("existing context file should be written");

    fs::write(test_context.work_file(), EXISTING_WORK_FILE_CONTENT)
        .expect("existing work file should be written");

    let overwrite_existing_files = true;

    setup_swelog_files_from_config(&test_context.config, overwrite_existing_files)
        .expect("swelog files should be overwritten");

    let context_file_contents =
        fs::read_to_string(test_context.context_file()).expect("context file should be readable");

    let work_file_contents =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(context_file_contents, DEFAULT_CONTEXT_FILE_CONTENT);
    assert_eq!(work_file_contents, DEFAULT_WORK_FILE_CONTENT);
    assert!(test_context.daily_log_directory().is_dir());
    assert!(test_context.weekly_log_directory().is_dir());

    drop(test_context.temporary_directory);
}

#[test]
fn setup_accepts_existing_log_directories_when_force_is_set() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.daily_log_directory())
        .expect("daily log directory should be created");

    fs::create_dir_all(test_context.weekly_log_directory())
        .expect("weekly log directory should be created");

    let overwrite_existing_files = true;

    setup_swelog_files_from_config(&test_context.config, overwrite_existing_files)
        .expect("existing log directories should be accepted");

    assert!(test_context.context_file().is_file());
    assert!(test_context.work_file().is_file());
    assert!(test_context.daily_log_directory().is_dir());
    assert!(test_context.weekly_log_directory().is_dir());

    drop(test_context.temporary_directory);
}
