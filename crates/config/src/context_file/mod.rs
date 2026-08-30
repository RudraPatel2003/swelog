use std::{
    fs,
    path::Path,
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

pub const CONTEXT_FILE_NAME: &str = "CONTEXT.md";

pub fn get_context_file_content(context_file: &Path) -> Result<Option<String>> {
    if !context_file.is_file() {
        return Ok(None);
    }

    let context_file_content = fs::read_to_string(context_file)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read context file at {}", context_file.display()))?;

    Ok(Some(context_file_content))
}

#[cfg(test)]
mod tests;
