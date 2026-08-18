use chrono::NaiveDate;
use clap::Parser;

use super::GithubArgs;

#[derive(Debug, Parser)]
struct TestCli {
    #[command(flatten)]
    github_args: GithubArgs,
}

#[test]
fn github_args_accepts_backfill_date() {
    let cli = TestCli::try_parse_from(["test", "--date", "07-04-2026"])
        .expect("valid backfill date should parse");

    assert_eq!(
        cli.github_args.date,
        Some(NaiveDate::from_ymd_opt(2026, 7, 4).expect("test date should be valid"))
    );
}

#[test]
fn github_args_allows_date_to_be_omitted() {
    let cli = TestCli::try_parse_from(["test"]).expect("omitted date should parse");

    assert_eq!(cli.github_args.date, None);
}

#[test]
fn github_args_rejects_invalid_date_format() {
    let error = TestCli::try_parse_from(["test", "--date", "2026-07-04"])
        .expect_err("invalid date format should fail");

    assert!(error.to_string().contains("expected MM-DD-YYYY"));
}
