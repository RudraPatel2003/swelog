mod errors;
use std::fs;

use config::{
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
    utils::ensure_swelog_file_exists,
};
use errors::SectionNotFound;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

pub fn log_to_work_file_from_config(
    swelog_config: &SwelogConfig,
    content: &str,
    section: &str,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_file_exists(&swelog_paths.work_file)?;

    let work_file_content =
        fs::read_to_string(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", swelog_paths.work_file.display())
        })?;

    let updated_work_file_content = append_to_section(&work_file_content, section, content)?;

    fs::write(&swelog_paths.work_file, updated_work_file_content).into_diagnostic().wrap_err_with(
        || format!("failed to write work file at {}", swelog_paths.work_file.display()),
    )?;

    Ok(())
}

fn append_to_section(markdown: &str, section: &str, content: &str) -> Result<String> {
    let heading = format!("## {section}");

    let start = markdown.find(&heading).ok_or(SectionNotFound { section: section.to_string() })?;
    let after_heading = &markdown[start + heading.len()..];

    let next_section = after_heading
        .find("\n## ")
        .map(|index| start + heading.len() + index)
        .unwrap_or(markdown.len());

    let mut result = String::new();

    result.push_str(markdown[..next_section].trim_end_matches('\n'));
    result.push('\n');
    result.push_str(content);
    result.push('\n');
    result.push_str(&markdown[next_section..]);

    Ok(result)
}

#[cfg(test)]
mod tests;
