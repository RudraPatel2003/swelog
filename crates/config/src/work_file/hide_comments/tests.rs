use std::path::PathBuf;

use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const EXISTING_FLAG_FILE_CONTENT: &str = "written by an earlier run";

struct TestContext {
    temporary_directory: TempDir,
    flag_file_path: PathBuf,
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let flag_file_path =
        temporary_directory.path().join(APP_NAME).join(HIDE_COMMENTS_FLAG_FILE_NAME);

    TestContext { temporary_directory, flag_file_path }
}

#[test]
fn hide_comments_flag_is_missing_before_it_is_set() {
    let TestContext { temporary_directory, flag_file_path } = get_test_context();

    assert!(!has_hide_comments_flag_at(&flag_file_path));

    drop(temporary_directory);
}

#[test]
fn set_hide_comments_flag_creates_parent_directories() {
    let TestContext { temporary_directory, flag_file_path } = get_test_context();

    set_hide_comments_flag_at(&flag_file_path).expect("hide comments flag should be written");

    assert!(flag_file_path.is_file());

    drop(temporary_directory);
}

#[test]
fn hide_comments_flag_is_present_after_it_is_set() {
    let TestContext { temporary_directory, flag_file_path } = get_test_context();

    set_hide_comments_flag_at(&flag_file_path).expect("hide comments flag should be written");

    assert!(has_hide_comments_flag_at(&flag_file_path));

    drop(temporary_directory);
}

#[test]
fn set_hide_comments_flag_leaves_an_existing_flag_untouched() {
    let TestContext { temporary_directory, flag_file_path } = get_test_context();

    set_hide_comments_flag_at(&flag_file_path).expect("hide comments flag should be written");

    fs::write(&flag_file_path, EXISTING_FLAG_FILE_CONTENT)
        .expect("existing flag file should be written");

    set_hide_comments_flag_at(&flag_file_path).expect("hide comments flag should stay unchanged");

    let flag_file_content =
        fs::read_to_string(&flag_file_path).expect("flag file should be readable");

    assert_eq!(flag_file_content, EXISTING_FLAG_FILE_CONTENT);

    drop(temporary_directory);
}
