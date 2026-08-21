use keyring::{
    Entry,
    Error as KeyringError,
};
use miette::Result;

use crate::{
    credential::Credential,
    errors::keyring_unavailable_error,
};

const KEYRING_SERVICE: &str = "swelog";

/// Reads the stored credential, ignoring any environment variable override.
pub fn read_credential(credential: Credential) -> Result<Option<String>> {
    match keyring_entry(credential)?.get_password() {
        Ok(secret) => Ok(Some(secret)),
        Err(KeyringError::NoEntry) => Ok(None),
        Err(error) => Err(keyring_unavailable_error(credential, &error)),
    }
}

/// Stores the credential, replacing any previously stored value.
pub fn write_credential(credential: Credential, secret: &str) -> Result<()> {
    keyring_entry(credential)?
        .set_password(secret)
        .map_err(|error| keyring_unavailable_error(credential, &error))
}

/// Removes the stored credential. Clearing one that is not stored succeeds, so
/// the command is safe to run at any time.
pub fn clear_credential(credential: Credential) -> Result<bool> {
    match keyring_entry(credential)?.delete_credential() {
        Ok(()) => Ok(true),
        Err(KeyringError::NoEntry) => Ok(false),
        Err(error) => Err(keyring_unavailable_error(credential, &error)),
    }
}

fn keyring_entry(credential: Credential) -> Result<Entry> {
    Entry::new(KEYRING_SERVICE, credential.keyring_username())
        .map_err(|error| keyring_unavailable_error(credential, &error))
}
