use async_trait::async_trait;
use credentials::{
    credential::Credential,
    store::CredentialStore,
};
use rmcp::transport::{
    AuthError,
    CredentialStore as McpCredentialStore,
    StoredCredentials,
};

/// Stores the Linear OAuth credentials wherever swelog keeps its other secrets.
#[derive(Clone)]
pub struct SwelogCredentialStore {
    credential_store: CredentialStore,
}

impl SwelogCredentialStore {
    pub const fn new(credential_store: CredentialStore) -> Self {
        Self { credential_store }
    }
}

#[async_trait]
impl McpCredentialStore for SwelogCredentialStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        let Some(stored_credentials_json) = self
            .credential_store
            .read(Credential::Linear)
            .map_err(|error| map_storage_error(&error))?
        else {
            return Ok(None);
        };

        let stored_credentials = serde_json::from_str(&stored_credentials_json)
            .map_err(|error| map_storage_error(&error))?;

        Ok(Some(stored_credentials))
    }

    async fn save(&self, stored_credentials: StoredCredentials) -> Result<(), AuthError> {
        let stored_credentials_json = serde_json::to_string(&stored_credentials)
            .map_err(|error| map_storage_error(&error))?;

        self.credential_store
            .write(Credential::Linear, &stored_credentials_json)
            .map_err(|error| map_storage_error(&error))
    }

    async fn clear(&self) -> Result<(), AuthError> {
        self.credential_store
            .clear(Credential::Linear)
            .map_err(|error| map_storage_error(&error))?;

        Ok(())
    }
}

fn map_storage_error(error: &impl ToString) -> AuthError {
    AuthError::InternalError(error.to_string())
}
