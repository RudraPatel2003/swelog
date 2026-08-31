use miette::Result;
use owo_colors::OwoColorize;

use crate::{
    swelog_config::{
        LanguageModelProvider,
        SwelogConfig,
    },
    utils::get_config_file_path,
};

const LABEL_WIDTH: usize = 21;
const NOT_CONFIGURED: &str = "Not configured";

pub fn print_config(config: &SwelogConfig) -> Result<()> {
    let config_file_path = get_config_file_path()?;

    let formatted_config = format_config(config);

    println!("Displaying config at {}:", config_file_path.display().cyan());
    println!();
    print!("{formatted_config}");

    Ok(())
}

fn format_config(config: &SwelogConfig) -> String {
    let mut output = String::new();

    output.push_str("Vault\n");
    output.push_str(&format_row(
        "Obsidian vault path",
        &config.obsidian_vault_path.display().to_string(),
    ));
    output.push_str(&format_row("Swelog folder name", &config.swelog_folder_name));
    output.push('\n');

    output.push_str("Files\n");
    output.push_str(&format_row("Work file", &config.work_file_name));
    output.push('\n');

    output.push_str("Logs\n");
    output.push_str(&format_row("Daily log folder", &config.daily_log_folder_name));
    output.push_str(&format_row("Weekly log folder", &config.weekly_log_folder_name));
    output.push('\n');

    output.push_str("Summarization\n");
    output.push_str(&format_row(
        "Provider",
        format_language_model_provider(config.language_model_provider),
    ));
    output
        .push_str(&format_row("Model", config.language_model.as_deref().unwrap_or(NOT_CONFIGURED)));
    output.push('\n');

    output.push_str("Integrations\n");
    output.push_str(&format_row(
        "Linear username",
        config.linear_username.as_deref().unwrap_or(NOT_CONFIGURED),
    ));

    output
}

const fn format_language_model_provider(
    language_model_provider: Option<LanguageModelProvider>,
) -> &'static str {
    match language_model_provider {
        Some(language_model_provider) => language_model_provider.label(),
        None => NOT_CONFIGURED,
    }
}

fn format_row(label: &str, value: &str) -> String {
    format!("  {label:<LABEL_WIDTH$}{value}\n")
}

#[cfg(test)]
mod tests;
