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
    LinearIssue,
    errors::LinearMcpRequestFailed,
    oauth::{
        LINEAR_MCP_URL,
        clear_linear_authorization,
        get_authorization_manager,
    },
    response::parse_issue_page,
};

const PAGE_SIZE: u64 = 50;

pub async fn get_active_assigned_issues(username: &str) -> Result<Vec<LinearIssue>> {
    let client = connect_to_linear_mcp().await?;

    let mut issues = fetch_all_issues(&client, username).await?;

    issues.retain(|issue| issue.status_type.is_active());

    client.cancel().await.map_err(|error| LinearMcpRequestFailed { message: error.to_string() })?;

    Ok(issues)
}

/// Connects to the Linear MCP server, discarding stored credentials and
/// reauthorizing once if the server rejects them.
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

            Err(error) => {
                return Err(LinearMcpRequestFailed { message: error.to_string() }.into());
            }
        }
    }
}

async fn fetch_all_issues(
    client: &RunningService<RoleClient, ()>,
    username: &str,
) -> Result<Vec<LinearIssue>> {
    let mut issues = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let result = client
            .call_tool(
                CallToolRequestParams::new("list_issues")
                    .with_arguments(list_issues_arguments(username, cursor.as_deref())),
            )
            .await
            .map_err(|error| LinearMcpRequestFailed { message: error.to_string() })?;

        let mut page = parse_issue_page(result)?;

        issues.append(&mut page.issues);

        let Some(next_cursor) = page.take_next_cursor() else {
            return Ok(issues);
        };

        if cursor.as_deref() == Some(next_cursor.as_str()) {
            return Err(LinearMcpRequestFailed {
                message: "Linear MCP returned a repeated pagination cursor".to_string(),
            }
            .into());
        }

        cursor = Some(next_cursor);
    }
}

fn list_issues_arguments(username: &str, cursor: Option<&str>) -> JsonObject {
    let mut arguments = JsonObject::new();

    arguments.insert("assignee".to_string(), Value::String(username.to_string()));
    arguments.insert("includeArchived".to_string(), Value::Bool(false));
    arguments.insert("limit".to_string(), Value::Number(PAGE_SIZE.into()));

    if let Some(cursor) = cursor {
        arguments.insert("cursor".to_string(), Value::String(cursor.to_string()));
    }

    arguments
}
