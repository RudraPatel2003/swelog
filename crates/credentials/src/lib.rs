pub mod errors;
mod prompt;

use std::env;

use clap::ValueEnum;
use keyring::{
    Entry,
    Error as KeyringError,
};
use miette::Result;

use crate::{
    errors::{
        KeyringUnavailable,
        MissingCredential,
    },
    prompt::{
        is_interactive_terminal,
        prompt_for_secret,
    },
};

const KEYRING_SERVICE: &str = "swelog";

/// A secret swelog stores in the operating system keyring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Credential {
    Github,
    OpenAi,
    OpenRouter,
    Linear,
}

impl Credential {
    pub const ALL_CREDENTIALS: [Self; 4] =
        [Self::Github, Self::OpenAi, Self::OpenRouter, Self::Linear];

    /// The name shown to the user in prompts, errors, and `swelog auth status`.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Github => "GitHub token",
            Self::OpenAi => "OpenAI API key",
            Self::OpenRouter => "OpenRouter API key",
            Self::Linear => "Linear authorization",
        }
    }

    /// The value accepted by `swelog auth clear`.
    #[must_use]
    pub const fn command_name(self) -> &'static str {
        match self {
            Self::Github => "github",
            Self::OpenAi => "open-ai",
            Self::OpenRouter => "open-router",
            Self::Linear => "linear",
        }
    }

    /// The environment variable that overrides the stored credential, when one
    /// exists. Linear is authorized through the browser, so it has none.
    #[must_use]
    pub const fn environment_variable(self) -> Option<&'static str> {
        match self {
            Self::Github => Some("GITHUB_TOKEN"),
            Self::OpenAi => Some("OPENAI_API_KEY"),
            Self::OpenRouter => Some("OPENROUTER_API_KEY"),
            Self::Linear => None,
        }
    }

    /// Where the user can obtain this credential. Linear is authorized through
    /// the browser rather than typed in, so it has no instructions.
    #[must_use]
    const fn prompt_instructions(self) -> Option<&'static str> {
        match self {
            Self::Github => {
                Some("Create one at https://github.com/settings/tokens with the `repo` scope.")
            }
            Self::OpenAi => Some("Create one at https://platform.openai.com/api-keys."),
            Self::OpenRouter => Some("Create one at https://openrouter.ai/keys."),
            Self::Linear => None,
        }
    }

    const fn keyring_username(self) -> &'static str {
        match self {
            Self::Github => "github-token",
            Self::OpenAi => "openai-api-key",
            Self::OpenRouter => "openrouter-api-key",
            Self::Linear => "linear-oauth",
        }
    }

    fn keyring_entry(self) -> Result<Entry> {
        Entry::new(KEYRING_SERVICE, self.keyring_username())
            .map_err(|error| self.keyring_unavailable_error(&error))
    }

    fn keyring_unavailable_error(self, error: &KeyringError) -> miette::Report {
        KeyringUnavailable { label: self.label(), message: error.to_string() }.into()
    }

    fn missing_credential_error(self) -> miette::Report {
        let Some(environment_variable) = self.environment_variable() else {
            return MissingCredential {
                label: self.label(),
                help: format!(
                    "run `swelog auth clear {}` and then `swelog fetch linear` to authorize again.",
                    self.command_name()
                ),
            }
            .into();
        };

        let help = format!(
            "run swelog from a terminal to enter it, or set the {environment_variable} environment variable. If a stored credential is stale, run `swelog auth clear {}`.",
            self.command_name()
        );

        MissingCredential { label: self.label(), help }.into()
    }
}

/// Reads the stored credential, ignoring any environment variable override.
pub fn read_credential(credential: Credential) -> Result<Option<String>> {
    match credential.keyring_entry()?.get_password() {
        Ok(secret) => Ok(Some(secret)),
        Err(KeyringError::NoEntry) => Ok(None),
        Err(error) => Err(credential.keyring_unavailable_error(&error)),
    }
}

/// Stores the credential, replacing any previously stored value.
pub fn write_credential(credential: Credential, secret: &str) -> Result<()> {
    credential
        .keyring_entry()?
        .set_password(secret)
        .map_err(|error| credential.keyring_unavailable_error(&error))
}

/// Removes the stored credential. Clearing a credential that is not stored
/// succeeds so the command can be run safely at any time.
pub fn clear_credential(credential: Credential) -> Result<bool> {
    match credential.keyring_entry()?.delete_credential() {
        Ok(()) => Ok(true),
        Err(KeyringError::NoEntry) => Ok(false),
        Err(error) => Err(credential.keyring_unavailable_error(&error)),
    }
}

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
        return Err(credential.missing_credential_error());
    };

    let secret = prompt_for_secret(credential.label(), instructions)?;

    if secret.is_empty() {
        return Err(credential.missing_credential_error());
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
