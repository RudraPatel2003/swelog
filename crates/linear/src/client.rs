use chrono::{
    Local,
    NaiveDate,
};
use miette::Result;
use reqwest::Client;
use rmcp::{
    RoleClient,
    ServiceExt,
    model::{
        CallToolRequestParams,
        JsonObject,
    },
    service::RunningService,
    transport::{
        AuthClient,
        StreamableHttpClientTransport,
        streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use serde_json::Value;

use crate::{
    activity::{
        get_updated_after_filter,
        was_issue_worked_on,
    },
    errors::LinearMcpRequestFailed,
    issue::LinearIssue,
    oauth::{
        LINEAR_MCP_URL,
        clear_linear_authorization,
        get_authorization_manager,
    },
    response::{
        LinearIssuePage,
        parse_issue_page,
    },
};

const PAGE_SIZE: u64 = 50;

/// Requesting fields explicitly keeps the response small and guarantees the
/// activity timestamps that date filtering depends on are present.
const ISSUE_FIELDS: [&str; 10] = [
    "id",
    "title",
    "url",
    "status",
    "statusType",
    "createdAt",
    "startedAt",
    "completedAt",
    "canceledAt",
    "updatedAt",
];

pub async fn get_active_assigned_issues(username: &str) -> Result<Vec<LinearIssue>> {
    let client = connect_to_linear_mcp().await?;

    let mut issues = fetch_all_issues(&client, username, None).await?;

    issues.retain(|issue| issue.status_type.is_active());

    disconnect_from_linear_mcp(client).await?;

    Ok(issues)
}

/// Fetches the issues assigned to `username` that Linear recorded activity on
/// during `date`.
///
/// Completed and canceled issues are kept, because finishing an issue is the
/// kind of work a past day's log should record.
pub async fn get_assigned_issues_worked_on(
    username: &str,
    date: &NaiveDate,
) -> Result<Vec<LinearIssue>> {
    let updated_after = get_updated_after_filter(*date)?;

    let client = connect_to_linear_mcp().await?;

    let mut issues = fetch_all_issues(&client, username, Some(&updated_after)).await?;

    issues.retain(|issue| was_issue_worked_on(issue, *date, &Local));

    disconnect_from_linear_mcp(client).await?;

    Ok(issues)
}

/// Connects to the Linear MCP server, reauthorizing once if the server rejects
/// the stored credentials.
async fn connect_to_linear_mcp() -> Result<RunningService<RoleClient, ()>> {
    let mut reauthorization_attempted = false;

    loop {
        let authorization_manager = get_authorization_manager().await?;
        let authorized_client = AuthClient::new(Client::new(), authorization_manager);
        let transport = StreamableHttpClientTransport::with_client(
            authorized_client,
            StreamableHttpClientTransportConfig::with_uri(LINEAR_MCP_URL),
        );

        match ().serve(transport).await {
            Ok(client) => return Ok(client),

            Err(error) if error.is_authorization_required() && !reauthorization_attempted => {
                clear_linear_authorization()?;

                reauthorization_attempted = true;
            }

            Err(error) => return Err(linear_mcp_request_failed(&error)),
        }
    }
}

async fn disconnect_from_linear_mcp(client: RunningService<RoleClient, ()>) -> Result<()> {
    client.cancel().await.map_err(|error| linear_mcp_request_failed(&error))?;

    Ok(())
}

async fn fetch_all_issues(
    client: &RunningService<RoleClient, ()>,
    username: &str,
    updated_after: Option<&str>,
) -> Result<Vec<LinearIssue>> {
    let mut issues = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let mut page = fetch_issue_page(client, username, cursor.as_deref(), updated_after).await?;

        issues.append(&mut page.issues);

        let Some(next_cursor) = page.take_next_cursor() else {
            return Ok(issues);
        };

        if cursor.as_deref() == Some(next_cursor.as_str()) {
            return Err(linear_mcp_request_failed(
                &"Linear MCP returned a repeated pagination cursor",
            ));
        }

        cursor = Some(next_cursor);
    }
}

async fn fetch_issue_page(
    client: &RunningService<RoleClient, ()>,
    username: &str,
    cursor: Option<&str>,
    updated_after: Option<&str>,
) -> Result<LinearIssuePage> {
    let arguments = list_issues_arguments(username, cursor, updated_after);

    let result = client
        .call_tool(CallToolRequestParams::new("list_issues").with_arguments(arguments))
        .await
        .map_err(|error| linear_mcp_request_failed(&error))?;

    parse_issue_page(result)
}

fn list_issues_arguments(
    username: &str,
    cursor: Option<&str>,
    updated_after: Option<&str>,
) -> JsonObject {
    let mut arguments = JsonObject::new();

    arguments.insert("assignee".to_string(), Value::String(username.to_string()));
    arguments.insert("includeArchived".to_string(), Value::Bool(false));
    arguments.insert("limit".to_string(), Value::Number(PAGE_SIZE.into()));
    arguments.insert("fields".to_string(), issue_fields_argument());

    if let Some(cursor) = cursor {
        arguments.insert("cursor".to_string(), Value::String(cursor.to_string()));
    }

    if let Some(updated_after) = updated_after {
        arguments.insert("updatedAt".to_string(), Value::String(updated_after.to_string()));
    }

    arguments
}

fn issue_fields_argument() -> Value {
    Value::Array(
        ISSUE_FIELDS.iter().map(|field| Value::String((*field).to_string())).collect::<Vec<_>>(),
    )
}

fn linear_mcp_request_failed(error: &impl ToString) -> miette::Report {
    LinearMcpRequestFailed { message: error.to_string() }.into()
}
