use std::{
    collections::{
        HashMap,
        HashSet,
    },
    path::Path,
};

use miette::Result;
use reqwest::Client;
use rmcp::{
    ServiceExt,
    model::{
        CallToolRequestParams,
        JsonObject,
    },
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
        clear_linear_authorization,
        get_authorization_manager,
        linear_mcp_url,
    },
    response::{
        UnresolvedLinearIssue,
        parse_issue_page,
        parse_statuses,
        resolve_issue,
    },
};

const PAGE_SIZE: u64 = 50;

pub async fn get_active_assigned_issues(
    username: &str,
    credentials_file: &Path,
) -> Result<Vec<LinearIssue>> {
    let mut reauthorization_attempted = false;

    let client = loop {
        let authorization_manager = get_authorization_manager(credentials_file).await?;
        let http_client = Client::new();
        let authorized_client = AuthClient::new(http_client, authorization_manager);
        let transport = StreamableHttpClientTransport::with_client(
            authorized_client,
            StreamableHttpClientTransportConfig::with_uri(linear_mcp_url()),
        );

        match ().serve(transport).await {
            Ok(client) => break client,
            Err(error) if error.is_authorization_required() && !reauthorization_attempted => {
                clear_linear_authorization(credentials_file)?;
                reauthorization_attempted = true;
            }
            Err(error) => {
                return Err(LinearMcpRequestFailed { message: error.to_string() }.into());
            }
        }
    };

    let unresolved_issues = fetch_all_issues(&client, username).await?;
    let statuses = if unresolved_issues.iter().any(|issue| issue.status_type.is_none()) {
        fetch_statuses(&client).await?
    } else {
        HashMap::new()
    };
    let mut issues = unresolved_issues
        .into_iter()
        .map(|issue| resolve_issue(issue, &statuses))
        .collect::<Result<Vec<_>>>()?;

    issues.retain(|issue| issue.status_type.is_active());

    client.cancel().await.map_err(|error| LinearMcpRequestFailed { message: error.to_string() })?;

    Ok(issues)
}

async fn fetch_all_issues<S>(
    client: &rmcp::service::RunningService<rmcp::RoleClient, S>,
    username: &str,
) -> Result<Vec<UnresolvedLinearIssue>>
where
    S: rmcp::Service<rmcp::RoleClient>,
{
    let mut issues = Vec::new();
    let mut cursor: Option<String> = None;
    let mut seen_cursors = HashSet::new();

    loop {
        let mut arguments = JsonObject::new();
        arguments.insert("assignee".to_string(), Value::String(username.to_string()));
        arguments.insert("includeArchived".to_string(), Value::Bool(false));
        arguments.insert("limit".to_string(), Value::Number(PAGE_SIZE.into()));

        if let Some(cursor) = &cursor {
            arguments.insert("cursor".to_string(), Value::String(cursor.clone()));
        }

        let result = client
            .call_tool(CallToolRequestParams::new("list_issues").with_arguments(arguments))
            .await
            .map_err(|error| LinearMcpRequestFailed { message: error.to_string() })?;
        let page = parse_issue_page(result)?;

        issues.extend(page.issues);

        let Some(next_cursor) = page.next_cursor else {
            break;
        };

        if !seen_cursors.insert(next_cursor.clone()) {
            return Err(LinearMcpRequestFailed {
                message: "Linear MCP returned a repeated pagination cursor".to_string(),
            }
            .into());
        }

        cursor = Some(next_cursor);
    }

    Ok(issues)
}

async fn fetch_statuses<S>(
    client: &rmcp::service::RunningService<rmcp::RoleClient, S>,
) -> Result<HashMap<String, crate::LinearStatusType>>
where
    S: rmcp::Service<rmcp::RoleClient>,
{
    let result = client
        .call_tool(CallToolRequestParams::new("list_issue_statuses"))
        .await
        .map_err(|error| LinearMcpRequestFailed { message: error.to_string() })?;

    parse_statuses(result)
}
