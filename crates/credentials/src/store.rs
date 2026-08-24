use keyring::{
    Entry,
    Error as KeyringError,
};
use miette::Result;

use crate::{
    credential::Credential,
    errors::KeyringUnavailable,
};

const KEYRING_SERVICE: &str = "swelog";

pub fn read_credential(credential: Credential) -> Result<Option<String>> {
    match keyring_entry(credential)?.get_password() {
        Ok(secret) => Ok(Some(secret)),
        Err(KeyringError::NoEntry) => Ok(None),
        Err(error) => {
            let keyring_unavailable_error =
                KeyringUnavailable { label: credential.label(), message: error.to_string() };

            Err(keyring_unavailable_error.into())
        }
    }
}

pub fn write_credential(credential: Credential, secret: &str) -> Result<()> {
    keyring_entry(credential)?.set_password(secret).map_err(|error| {
        KeyringUnavailable { label: credential.label(), message: error.to_string() }.into()
    })
}

pub fn clear_credential(credential: Credential) -> Result<bool> {
    match keyring_entry(credential)?.delete_credential() {
        Ok(()) => Ok(true),
        Err(KeyringError::NoEntry) => Ok(false),
        Err(error) => {
            let keyring_unavailable_error =
                KeyringUnavailable { label: credential.label(), message: error.to_string() };

            Err(keyring_unavailable_error.into())
        }
    }
}

fn keyring_entry(credential: Credential) -> Result<Entry> {
    Entry::new(KEYRING_SERVICE, credential.keyring_username()).map_err(|error| {
        KeyringUnavailable { label: credential.label(), message: error.to_string() }.into()
    })
}
