use keyring::Error as KeyringError;
use miette::{
    Diagnostic,
    Report,
};
use thiserror::Error;

use crate::credential::Credential;

#[derive(Debug, Diagnostic, Error)]
#[error("{label} is not available")]
#[diagnostic(code(swelog::credentials::missing_credential))]
pub struct MissingCredential {
    pub label: &'static str,

    #[help]
    pub help: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to access the {label} credential in your operating system keyring")]
#[diagnostic(
    code(swelog::credentials::keyring_unavailable),
    help("unlock your keyring and try again: {message}")
)]
pub struct KeyringUnavailable {
    pub label: &'static str,
    pub message: String,
}

#[must_use]
pub fn keyring_unavailable_error(credential: Credential, error: &KeyringError) -> Report {
    KeyringUnavailable { label: credential.label(), message: error.to_string() }.into()
}

/// Linear is authorized through the browser, so its help points at that flow
/// rather than at an environment variable.
#[must_use]
pub fn missing_credential_error(credential: Credential) -> Report {
    let Some(environment_variable) = credential.environment_variable() else {
        return MissingCredential {
            label: credential.label(),
            help: format!(
                "run `swelog auth clear {}` and then `swelog fetch linear` to authorize again.",
                credential.command_name()
            ),
        }
        .into();
    };

    let help = format!(
        "run swelog from a terminal to enter it, or set the {environment_variable} environment variable. If a stored credential is stale, run `swelog auth clear {}`.",
        credential.command_name()
    );

    MissingCredential { label: credential.label(), help }.into()
}
