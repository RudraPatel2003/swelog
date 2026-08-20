use std::io::{
    IsTerminal,
    Write,
    stderr,
    stdin,
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

/// Returns `true` when swelog can ask the user for a credential without
/// blocking a scripted or CI invocation.
pub fn is_interactive_terminal() -> bool {
    stdin().is_terminal() && stderr().is_terminal()
}

/// Reads a secret from the terminal without echoing it.
pub fn prompt_for_secret(label: &str, instructions: &str) -> Result<String> {
    let mut error_output = stderr();

    writeln!(error_output, "{label} is not stored yet. {instructions}")
        .into_diagnostic()
        .wrap_err("failed to write the credential prompt")?;

    let secret = rpassword::prompt_password(format!("Enter your {label}: "))
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read the {label} from the terminal"))?;

    Ok(secret.trim().to_string())
}
