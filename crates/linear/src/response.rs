use std::collections::HashMap;

use miette::Result;
use rmcp::model::CallToolResult;
use serde_json::Value;

use crate::{
    LinearIssue,
    LinearStatusType,
    errors::UnsupportedLinearResponse,
};

#[derive(Debug)]
pub struct ParsedIssuePage {
    pub issues: Vec<UnresolvedLinearIssue>,
    pub next_cursor: Option<String>,
}

#[derive(Debug)]
pub struct UnresolvedLinearIssue {
    pub identifier: String,
    pub title: String,
    pub url: String,
    pub status_id: Option<String>,
    pub status_name: String,
    pub status_type: Option<LinearStatusType>,
}

pub fn parse_issue_page(result: CallToolResult) -> Result<ParsedIssuePage> {
    let value = extract_result_value(result)?;
    let issue_values = find_array(&value, &["issues", "nodes", "items"])
        .or_else(|| value.as_array())
        .ok_or_else(|| unsupported("issue response did not contain an issue array"))?;

    let issues = issue_values.iter().map(parse_issue).collect::<Result<Vec<_>>>()?;
    let next_cursor = find_string(&value, &["nextCursor", "next_cursor"])
        .or_else(|| {
            value
                .get("pageInfo")
                .and_then(|page_info| find_string(page_info, &["endCursor", "nextCursor"]))
        })
        .filter(|cursor| !cursor.is_empty());
    let has_next_page = find_bool(&value, &["hasNextPage", "has_next_page"])
        .or_else(|| {
            value.get("pageInfo").and_then(|page_info| find_bool(page_info, &["hasNextPage"]))
        })
        .unwrap_or_else(|| next_cursor.is_some());

    Ok(ParsedIssuePage { issues, next_cursor: has_next_page.then_some(next_cursor).flatten() })
}

pub fn parse_statuses(result: CallToolResult) -> Result<HashMap<String, LinearStatusType>> {
    let value = extract_result_value(result)?;
    let status_values = find_array(&value, &["statuses", "issueStatuses", "nodes", "items"])
        .or_else(|| value.as_array())
        .ok_or_else(|| unsupported("status response did not contain a status array"))?;
    let mut statuses = HashMap::new();

    for status in status_values {
        let status_type = required_string(status, &["type"])?;
        let status_type = parse_status_type(&status_type);

        if let Some(status_id) = find_string(status, &["id"]) {
            statuses.insert(status_id, status_type.clone());
        }

        if let Some(status_name) = find_string(status, &["name"]) {
            statuses.insert(status_name, status_type);
        }
    }

    Ok(statuses)
}

pub fn resolve_issue(
    issue: UnresolvedLinearIssue,
    statuses: &HashMap<String, LinearStatusType>,
) -> Result<LinearIssue> {
    let status_type = issue
        .status_type
        .or_else(|| issue.status_id.as_ref().and_then(|status_id| statuses.get(status_id).cloned()))
        .or_else(|| statuses.get(&issue.status_name).cloned())
        .ok_or_else(|| unsupported(format!("status type was missing for {}", issue.identifier)))?;

    Ok(LinearIssue {
        identifier: issue.identifier,
        title: issue.title,
        url: issue.url,
        status_name: issue.status_name,
        status_type,
    })
}

fn parse_issue(value: &Value) -> Result<UnresolvedLinearIssue> {
    let status = value
        .get("status")
        .or_else(|| value.get("state"))
        .ok_or_else(|| unsupported("issue was missing its status"))?;
    let issue_status_type = find_string(value, &["statusType", "status_type"])
        .map(|status_type| parse_status_type(&status_type));

    let (status_id, status_name, status_type) = match status {
        Value::String(status_name) => (None, status_name.clone(), issue_status_type),
        Value::Object(_) => {
            let status_id = find_string(status, &["id"]);
            let status_name = required_string(status, &["name"])?;
            let status_type = find_string(status, &["type"])
                .map(|status_type| parse_status_type(&status_type))
                .or(issue_status_type);

            (status_id, status_name, status_type)
        }
        _ => return Err(unsupported("issue status had an unsupported shape")),
    };

    Ok(UnresolvedLinearIssue {
        identifier: parse_issue_identifier(value)?,
        title: required_string(value, &["title"])?,
        url: required_string(value, &["url"])?,
        status_id,
        status_name,
        status_type,
    })
}

fn parse_issue_identifier(value: &Value) -> Result<String> {
    find_string(value, &["identifier", "issueIdentifier", "issue_identifier"])
        .or_else(|| {
            find_string(value, &["id"]).filter(|identifier| is_issue_identifier(identifier))
        })
        .or_else(|| {
            find_string(value, &["url"])
                .as_deref()
                .and_then(issue_identifier_from_url)
                .map(ToString::to_string)
        })
        .ok_or_else(|| unsupported("missing identifier"))
}

fn issue_identifier_from_url(url: &str) -> Option<&str> {
    let (_, issue_path) = url.split_once("/issue/")?;
    let identifier = issue_path.split(['/', '?', '#']).next()?;

    is_issue_identifier(identifier).then_some(identifier)
}

fn is_issue_identifier(identifier: &str) -> bool {
    let Some((team_key, issue_number)) = identifier.rsplit_once('-') else {
        return false;
    };

    !team_key.is_empty()
        && team_key.chars().all(|character| character.is_ascii_alphanumeric())
        && !issue_number.is_empty()
        && issue_number.chars().all(|character| character.is_ascii_digit())
}

fn extract_result_value(result: CallToolResult) -> Result<Value> {
    if result.is_error == Some(true) {
        let message = result
            .content
            .iter()
            .filter_map(|content| content.as_text())
            .map(|text| text.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        return Err(unsupported(if message.is_empty() {
            "Linear MCP tool returned an error".to_string()
        } else {
            message
        }));
    }

    if let Some(value) = result.structured_content {
        return Ok(value);
    }

    for content in result.content {
        if let Some(text) = content.as_text() {
            if text.text.contains("No issues found") {
                return Ok(serde_json::json!({ "issues": [] }));
            }

            if let Ok(value) = serde_json::from_str(&text.text) {
                return Ok(value);
            }
        }
    }

    Err(unsupported("tool result did not contain structured JSON"))
}

fn find_array<'a>(value: &'a Value, keys: &[&str]) -> Option<&'a Vec<Value>> {
    for key in keys {
        if let Some(array) = value.get(key).and_then(Value::as_array) {
            return Some(array);
        }

        if let Some(array) =
            value.get("data").and_then(|data| data.get(key)).and_then(Value::as_array)
        {
            return Some(array);
        }
    }

    None
}

fn required_string(value: &Value, keys: &[&str]) -> Result<String> {
    find_string(value, keys).ok_or_else(|| unsupported(format!("missing {}", keys.join(" or "))))
}

fn find_string(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| value.get(key).and_then(Value::as_str).map(ToString::to_string))
}

fn find_bool(value: &Value, keys: &[&str]) -> Option<bool> {
    keys.iter().find_map(|key| value.get(key).and_then(Value::as_bool))
}

fn parse_status_type(status_type: &str) -> LinearStatusType {
    match status_type.to_ascii_lowercase().as_str() {
        "started" => LinearStatusType::Started,
        "unstarted" => LinearStatusType::Unstarted,
        "backlog" => LinearStatusType::Backlog,
        "completed" => LinearStatusType::Completed,
        "canceled" | "cancelled" => LinearStatusType::Canceled,
        _ => LinearStatusType::Other(status_type.to_string()),
    }
}

fn unsupported(message: impl Into<String>) -> miette::Report {
    UnsupportedLinearResponse { message: message.into() }.into()
}

#[cfg(test)]
mod tests;
