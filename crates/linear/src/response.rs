use miette::Result;
use rmcp::model::CallToolResult;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    LinearIssue,
    errors::UnsupportedLinearResponse,
};

const NO_ISSUES_FOUND_MESSAGE: &str = "No issues found";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearIssuePage {
    #[serde(default)]
    pub issues: Vec<LinearIssue>,

    /// The Linear MCP server returns this as `cursor`; the alias keeps older
    /// `nextCursor` responses working.
    #[serde(default, rename = "cursor", alias = "nextCursor")]
    pub next_cursor: Option<String>,

    #[serde(default)]
    pub has_next_page: Option<bool>,
}

impl LinearIssuePage {
    /// Returns the cursor for the following page, or `None` when this is the
    /// last page.
    pub fn take_next_cursor(&mut self) -> Option<String> {
        if self.has_next_page == Some(false) {
            return None;
        }

        self.next_cursor.take().filter(|next_cursor| !next_cursor.is_empty())
    }
}

pub fn parse_issue_page(result: CallToolResult) -> Result<LinearIssuePage> {
    let value = extract_result_value(result)?;

    serde_json::from_value(value).map_err(|error| {
        unsupported_response_error(format!(
            "issue response did not match the expected shape: {error}"
        ))
    })
}

fn extract_result_value(result: CallToolResult) -> Result<Value> {
    if result.is_error == Some(true) {
        let message = collect_result_text(&result);

        return Err(unsupported_response_error(if message.is_empty() {
            "Linear MCP tool returned an error".to_string()
        } else {
            message
        }));
    }

    if let Some(value) = result.structured_content {
        return Ok(value);
    }

    let text = collect_result_text(&result);

    if text.contains(NO_ISSUES_FOUND_MESSAGE) {
        return Ok(serde_json::json!({ "issues": [] }));
    }

    serde_json::from_str(&text).map_err(|error| {
        unsupported_response_error(format!("tool result did not contain structured JSON: {error}"))
    })
}

fn collect_result_text(result: &CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|content| content.as_text())
        .map(|text| text.text.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn unsupported_response_error(message: impl Into<String>) -> miette::Report {
    UnsupportedLinearResponse { message: message.into() }.into()
}

#[cfg(test)]
mod tests;
