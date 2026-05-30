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

const WORK_ITEM: &str = "Meeting with manager";

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
fn log_work_item_appends_bullet_to_work_file() {
    let test_context = get_test_context();

    test_context.write_work_file("# Today's Work\n\n## Notes\n");

    log_work_item_from_config(&test_context.config, WORK_ITEM)
        .expect("work item should be appended");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_content, "# Today's Work\n\n## Notes\n- Meeting with manager\n");

    drop(test_context.temporary_directory);
}

#[test]
fn log_work_item_inserts_newline_when_work_file_has_no_trailing_newline() {
    let test_context = get_test_context();

    test_context.write_work_file("# Today's Work");

    log_work_item_from_config(&test_context.config, WORK_ITEM)
        .expect("work item should be appended");

    let work_file_content =
        fs::read_to_string(test_context.work_file()).expect("work file should be readable");

    assert_eq!(work_file_content, "# Today's Work\n- Meeting with manager\n");

    drop(test_context.temporary_directory);
}

#[test]
fn log_work_item_fails_when_work_file_is_missing() {
    let test_context = get_test_context();

    fs::create_dir_all(test_context.swelog_directory())
        .expect("swelog directory should be created");

    let error = log_work_item_from_config(&test_context.config, WORK_ITEM)
        .expect_err("missing work file should fail");

    let error =
        error.downcast_ref::<SwelogFileNotFound>().expect("error should be SwelogFileNotFound");

    assert_eq!(error.swelog_path, test_context.work_file());

    drop(test_context.temporary_directory);
}
