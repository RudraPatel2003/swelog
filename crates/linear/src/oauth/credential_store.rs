use async_trait::async_trait;
use credentials::{
    Credential,
    clear_credential,
    read_credential,
    write_credential,
};
use rmcp::transport::{
    AuthError,
    CredentialStore,
    StoredCredentials,
};

/// Stores the Linear OAuth credentials in the operating system keyring.
#[derive(Clone, Copy)]
pub struct KeyringCredentialStore;

#[async_trait]
impl CredentialStore for KeyringCredentialStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        let Some(stored_credentials) =
            read_credential(Credential::Linear).map_err(|error| map_storage_error(&error))?
        else {
            return Ok(None);
        };

        let stored_credentials =
            serde_json::from_str(&stored_credentials).map_err(|error| map_storage_error(&error))?;

        Ok(Some(stored_credentials))
    }

    async fn save(&self, stored_credentials: StoredCredentials) -> Result<(), AuthError> {
        let stored_credentials = serde_json::to_string(&stored_credentials)
            .map_err(|error| map_storage_error(&error))?;

        write_credential(Credential::Linear, &stored_credentials)
            .map_err(|error| map_storage_error(&error))
    }

    async fn clear(&self) -> Result<(), AuthError> {
        clear_credential(Credential::Linear).map_err(|error| map_storage_error(&error))?;

        Ok(())
    }
}

fn map_storage_error(error: &impl ToString) -> AuthError {
    AuthError::InternalError(error.to_string())
}
