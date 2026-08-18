use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to authorize with Linear: {message}")]
#[diagnostic(
    code(swelog::linear::authorization_failed),
    help("retry the command and complete authorization using the displayed URL")
)]
pub struct LinearAuthorizationFailed {
    pub message: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Timed out waiting for Linear authorization")]
#[diagnostic(
    code(swelog::linear::authorization_timed_out),
    help("run the command again and complete authorization in the browser")
)]
pub struct LinearAuthorizationTimedOut;

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to store Linear OAuth credentials at {path}")]
#[diagnostic(code(swelog::linear::credential_store_failed))]
pub struct LinearCredentialStoreFailed {
    pub path: PathBuf,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to communicate with the Linear MCP server: {message}")]
#[diagnostic(
    code(swelog::linear::mcp_request_failed),
    help("check your network connection and Linear authorization, then try again")
)]
pub struct LinearMcpRequestFailed {
    pub message: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Linear MCP returned an unsupported response: {message}")]
#[diagnostic(code(swelog::linear::unsupported_response))]
pub struct UnsupportedLinearResponse {
    pub message: String,
}
