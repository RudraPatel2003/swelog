mod callback_server;
mod credential_store;

use std::time::Duration;

use credentials::{
    Credential,
    clear_credential,
};
use miette::Result;
use rmcp::transport::{
    AuthError,
    AuthorizationManager,
    AuthorizationRequest,
    AuthorizationSession,
};
use tokio::time::timeout;

use crate::{
    errors::{
        LinearAuthorizationFailed,
        LinearAuthorizationTimedOut,
    },
    oauth::{
        callback_server::CallbackServer,
        credential_store::KeyringCredentialStore,
    },
};

pub const LINEAR_MCP_URL: &str = "https://mcp.linear.app/mcp/readonly";

const AUTHORIZATION_TIMEOUT: Duration = Duration::from_secs(300);

pub async fn get_authorization_manager() -> Result<AuthorizationManager> {
    let mut authorization_manager = AuthorizationManager::new(LINEAR_MCP_URL)
        .await
        .map_err(|error| map_authorization_error(&error))?;

    authorization_manager.set_credential_store(KeyringCredentialStore);

    if authorization_manager
        .initialize_from_store()
        .await
        .map_err(|error| map_authorization_error(&error))?
    {
        return Ok(authorization_manager);
    }

    authorize_interactively(authorization_manager).await
}

pub fn clear_linear_authorization() -> Result<()> {
    clear_credential(Credential::Linear)?;

    Ok(())
}

async fn authorize_interactively(
    mut authorization_manager: AuthorizationManager,
) -> Result<AuthorizationManager> {
    let callback_server = CallbackServer::bind().await?;

    let metadata = authorization_manager
        .resolve_metadata()
        .await
        .map_err(|error| map_authorization_error(&error))?
        .metadata;

    authorization_manager.set_metadata(metadata);

    let authorization_request =
        AuthorizationRequest::new(callback_server.redirect_uri()).with_client_name("swelog");

    let authorization_session =
        AuthorizationSession::new(authorization_manager, authorization_request)
            .await
            .map_err(|(_, error)| map_authorization_error(&error))?;

    let authorization_url = authorization_session.get_authorization_url();

    println!("Authorize swelog with Linear using this URL:\n{authorization_url}\n");

    if webbrowser::open(authorization_url).is_err() {
        println!("Unable to open a browser automatically. Open the URL above manually.");
    }

    let callback_url = timeout(AUTHORIZATION_TIMEOUT, callback_server.receive_callback_url())
        .await
        .map_err(|_| LinearAuthorizationTimedOut)??;

    authorization_session
        .handle_callback_url(&callback_url)
        .await
        .map_err(|error| map_authorization_error(&error))?;

    Ok(authorization_session.auth_manager)
}

fn map_authorization_error(error: &AuthError) -> miette::Report {
    LinearAuthorizationFailed { message: error.to_string() }.into()
}
