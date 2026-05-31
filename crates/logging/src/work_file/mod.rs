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

pub fn append_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    content: &str,
    section: &str,
) -> Result<()> {
    update_work_file_section_from_config(swelog_config, content, section, append_to_section)
}

pub fn overwrite_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    content: &str,
    section: &str,
) -> Result<()> {
    update_work_file_section_from_config(swelog_config, content, section, overwrite_section)
}

fn update_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    content: &str,
    section: &str,
    update_section: fn(&str, &str, &str) -> Result<String>,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_file_exists(&swelog_paths.work_file)?;

    let work_file_content =
        fs::read_to_string(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", swelog_paths.work_file.display())
        })?;

    let updated_work_file_content = update_section(&work_file_content, section, content)?;

    fs::write(&swelog_paths.work_file, updated_work_file_content).into_diagnostic().wrap_err_with(
        || format!("failed to write work file at {}", swelog_paths.work_file.display()),
    )?;

    Ok(())
}

fn append_to_section(markdown: &str, section: &str, content: &str) -> Result<String> {
    let section_bounds = find_section_bounds(markdown, section)?;

    let mut result = String::new();

    result.push_str(markdown[..section_bounds.next_section].trim_end_matches('\n'));
    result.push('\n');
    result.push_str(content);
    result.push('\n');
    result.push_str(&markdown[section_bounds.next_section..]);

    Ok(result)
}

fn overwrite_section(markdown: &str, section: &str, content: &str) -> Result<String> {
    let section_bounds = find_section_bounds(markdown, section)?;

    let mut result = String::new();

    result.push_str(markdown[..section_bounds.content_start].trim_end_matches('\n'));
    result.push('\n');
    result.push_str(content);
    result.push('\n');
    result.push_str(&markdown[section_bounds.next_section..]);

    Ok(result)
}

struct SectionBounds {
    content_start: usize,
    next_section: usize,
}

fn find_section_bounds(markdown: &str, section: &str) -> Result<SectionBounds> {
    let heading = format!("## {section}");

    let start = markdown.find(&heading).ok_or(SectionNotFound { section: section.to_string() })?;

    let content_start = start + heading.len();

    let after_heading = &markdown[content_start..];

    let next_section =
        after_heading.find("\n## ").map(|index| content_start + index).unwrap_or(markdown.len());

    Ok(SectionBounds { content_start, next_section })
}

#[cfg(test)]
mod tests;
