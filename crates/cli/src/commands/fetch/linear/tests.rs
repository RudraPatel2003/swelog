use clap::Parser;

use super::*;

#[derive(Debug, Parser)]
struct TestCli {
    #[command(flatten)]
    linear_args: LinearArgs,
}

#[test]
fn linear_args_fetches_when_subcommand_is_omitted() {
    let cli = TestCli::try_parse_from(["test"]).expect("omitted subcommand should parse");

    assert!(cli.linear_args.command.is_none());
}

#[test]
fn linear_args_accepts_logout_subcommand() {
    let cli = TestCli::try_parse_from(["test", "logout"]).expect("logout should parse");

    assert!(matches!(cli.linear_args.command, Some(LinearCommands::Logout)));
}

#[test]
fn format_linear_issues_groups_and_orders_active_statuses() {
    let issues = vec![
        issue("ENG-1", "Todo issue", "Todo", LinearStatusType::Unstarted),
        issue("ENG-2", "Started issue", "In Progress", LinearStatusType::Started),
        issue("ENG-3", "Backlog issue", "Backlog", LinearStatusType::Backlog),
    ];

    let markdown = format_linear_issues(&issues);

    assert_eq!(
        markdown,
        "### In Progress\n- [ENG-2](https://linear.app/issue/ENG-2) Started issue\n\n### Todo\n- [ENG-1](https://linear.app/issue/ENG-1) Todo issue\n\n### Backlog\n- [ENG-3](https://linear.app/issue/ENG-3) Backlog issue"
    );
}

#[test]
fn format_linear_issues_collapses_whitespace_and_escapes_link_text() {
    let issues =
        vec![issue("ENG-[1]", "Fix [OAuth]\ncallback", "In Progress", LinearStatusType::Started)];

    let markdown = format_linear_issues(&issues);

    assert!(markdown.contains("[ENG-\\[1\\]]"));
    assert!(markdown.contains("Fix \\[OAuth\\] callback"));
}

fn issue(
    identifier: &str,
    title: &str,
    status_name: &str,
    status_type: LinearStatusType,
) -> LinearIssue {
    LinearIssue {
        identifier: identifier.to_string(),
        title: title.to_string(),
        url: format!("https://linear.app/issue/{identifier}"),
        status_name: status_name.to_string(),
        status_type,
    }
}
