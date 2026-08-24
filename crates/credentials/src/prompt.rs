use std::io::{
    Write,
    stderr,
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

/// Reads a secret from the terminal without echoing it.
pub fn prompt_for_secret(label: &str, instructions: &str) -> Result<String> {
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
