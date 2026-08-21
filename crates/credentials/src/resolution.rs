use std::env;

use miette::Result;

use crate::{
    credential::Credential,
    errors::missing_credential_error,
    prompt::{
        is_interactive_terminal,
        prompt_for_secret,
    },
    store::{
        read_credential,
        write_credential,
    },
};

/// Resolves a credential from the environment, then the keyring, prompting for
/// it and storing the answer when swelog is running interactively.
pub fn get_or_prompt_for_credential(credential: Credential) -> Result<String> {
    if let Some(secret) = read_credential_from_environment(credential) {
        return Ok(secret);
    }

    if let Some(secret) = read_credential(credential)? {
        return Ok(secret);
    }

    let (Some(instructions), true) = (credential.prompt_instructions(), is_interactive_terminal())
    else {
        return Err(missing_credential_error(credential));
    };

    let secret = prompt_for_secret(credential.label(), instructions)?;

    if secret.is_empty() {
        return Err(missing_credential_error(credential));
    }

    write_credential(credential, &secret)?;

    Ok(secret)
}

/// Returns the environment variable overriding this credential, when it is set
/// to a non-empty value.
#[must_use]
pub fn read_credential_from_environment(credential: Credential) -> Option<String> {
    let environment_variable = credential.environment_variable()?;

    env::var(environment_variable)
        .ok()
        .map(|secret| secret.trim().to_string())
        .filter(|secret| !secret.is_empty())
}
