use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to authorize with Linear: {message}")]
#[diagnostic(
    code(swelog::linear::authorization_failed),
    help("run `swelog auth clear linear`, then run `swelog fetch linear` to authorize again")
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
#[error("Failed to communicate with the Linear MCP server: {message}")]
#[diagnostic(
    code(swelog::linear::mcp_request_failed),
    help(
        "check your network connection, or run `swelog auth clear linear` if your authorization is stale"
    )
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
