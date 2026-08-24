use std::{
    env,
    io::{
        Write,
        stderr,
    },
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::{
    credential::Credential,
    errors::{
        MissingAuthorization,
        MissingCredential,
    },
    store::{
        read_credential,
        write_credential,
    },
};

pub fn get_or_prompt_for_credential(credential: Credential) -> Result<String> {
    if let Some(secret) = read_credential_from_environment(credential) {
        return Ok(secret);
    }

    if let Some(secret) = read_credential(credential)? {
        return Ok(secret);
    }

    let (Some(environment_variable), Some(instructions)) =
        (credential.environment_variable(), credential.prompt_instructions())
    else {
        let missing_authorization_error = MissingAuthorization {
            label: credential.label(),
            command_name: credential.command_name(),
        };

        return Err(missing_authorization_error.into());
    };

    let secret = prompt_for_secret(credential.label(), instructions)?;

    if secret.is_empty() {
        let missing_credential_error = MissingCredential {
            label: credential.label(),
            environment_variable,
            command_name: credential.command_name(),
        };

        return Err(missing_credential_error.into());
    }

    write_credential(credential, &secret)?;

    Ok(secret)
}

#[must_use]
pub fn read_credential_from_environment(credential: Credential) -> Option<String> {
    let environment_variable = credential.environment_variable()?;

    env::var(environment_variable)
        .ok()
        .map(|secret| secret.trim().to_string())
        .filter(|secret| !secret.is_empty())
}

fn prompt_for_secret(label: &str, instructions: &str) -> Result<String> {
    let mut error_output = stderr();

    writeln!(error_output, "{label} is not stored yet. {instructions}")
        .into_diagnostic()
        .wrap_err("failed to write the credential prompt")?;

    let secret = rpassword::prompt_password(format!("Enter your {label}: "))
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read the {label} from the terminal"))?;

    let trimmed_secret = secret.trim().to_string();

    Ok(trimmed_secret)
}
