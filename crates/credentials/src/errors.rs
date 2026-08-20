use miette::Diagnostic;
use thiserror::Error;

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
