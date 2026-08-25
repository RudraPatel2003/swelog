mod errors;
mod slicing;

use std::{
    fs,
    ops::Range,
};

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
use pulldown_cmark::{
    Event,
    HeadingLevel,
    Parser,
    Tag,
    TagEnd,
};
use slicing::{
    slice_from,
    slice_up_to,
};

/// Integration sections are inserted directly above this section so the user's
/// own notes stay at the bottom of the work file.
const LOG_SECTION_TITLE: &str = "Log";

pub fn append_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    content: &str,
    section: &str,
) -> Result<()> {
    update_work_file_from_config(swelog_config, |work_file_content| {
        append_to_section(work_file_content, section, content)
    })
}

pub fn upsert_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    section_title: &str,
    content: &str,
) -> Result<()> {
    update_work_file_from_config(swelog_config, |work_file_content| {
        Ok(upsert_section(work_file_content, section_title, content))
    })
}

pub fn remove_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    section_title: &str,
) -> Result<()> {
    update_work_file_from_config(swelog_config, |work_file_content| {
        Ok(remove_section(work_file_content, section_title))
    })
}

fn update_work_file_from_config(
    swelog_config: &SwelogConfig,
    update_work_file: impl FnOnce(&str) -> Result<String>,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_file_exists(&swelog_paths.work_file)?;

    let work_file_content =
        fs::read_to_string(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", swelog_paths.work_file.display())
        })?;

    let updated_work_file_content = update_work_file(&work_file_content)?;

    fs::write(&swelog_paths.work_file, updated_work_file_content).into_diagnostic().wrap_err_with(
        || format!("failed to write work file at {}", swelog_paths.work_file.display()),
    )?;

    Ok(())
}

fn append_to_section(markdown: &str, section: &str, content: &str) -> Result<String> {
    let section_bounds = find_section_bounds(markdown, section)?;
    let suffix = slice_from(markdown, section_bounds.end).trim_start_matches('\n');

    let mut result = String::new();

    result.push_str(slice_up_to(markdown, section_bounds.end).trim_end_matches('\n'));
    result.push('\n');
    result.push_str(content);

    if suffix.is_empty() {
        result.push('\n');
    } else {
        result.push_str("\n\n");
        result.push_str(suffix);
    }

    Ok(result)
}

fn upsert_section(markdown: &str, section_title: &str, content: &str) -> String {
    let section_block = format_section(section_title, content);

    let section_bounds = find_optional_section_bounds(markdown, section_title)
        .unwrap_or_else(|| insertion_bounds_above_log(markdown));

    replace_block(markdown, section_bounds, &section_block)
}

fn remove_section(markdown: &str, section_title: &str) -> String {
    find_optional_section_bounds(markdown, section_title).map_or_else(
        || markdown.to_string(),
        |section_bounds| remove_block(markdown, section_bounds),
    )
}

fn insertion_bounds_above_log(markdown: &str) -> Range<usize> {
    let insertion_index = find_optional_section_bounds(markdown, LOG_SECTION_TITLE)
        .map_or(markdown.len(), |log_section_bounds| log_section_bounds.start);

    insertion_index..insertion_index
}

fn format_section(section_title: &str, content: &str) -> String {
    format!("## {section_title}\n{}", content.trim_matches('\n'))
}

fn replace_block(markdown: &str, range: Range<usize>, block: &str) -> String {
    let prefix = slice_up_to(markdown, range.start).trim_end_matches('\n');
    let suffix = slice_from(markdown, range.end).trim_start_matches('\n');

    match (prefix.is_empty(), suffix.is_empty()) {
        (true, true) => format!("{block}\n"),
        (true, false) => format!("{block}\n\n{suffix}"),
        (false, true) => format!("{prefix}\n\n{block}\n"),
        (false, false) => format!("{prefix}\n\n{block}\n\n{suffix}"),
    }
}

fn remove_block(markdown: &str, range: Range<usize>) -> String {
    let prefix = slice_up_to(markdown, range.start).trim_end_matches('\n');
    let suffix = slice_from(markdown, range.end).trim_start_matches('\n');

    match (prefix.is_empty(), suffix.is_empty()) {
        (true, true) => String::new(),
        (true, false) => suffix.to_string(),
        (false, true) => format!("{prefix}\n"),
        (false, false) => format!("{prefix}\n\n{suffix}"),
    }
}

struct Heading {
    level: HeadingLevel,
    title: String,
    range: Range<usize>,
}

fn find_section_bounds(markdown: &str, section: &str) -> Result<Range<usize>> {
    find_optional_section_bounds(markdown, section)
        .ok_or_else(|| SectionNotFound { section: section.to_string() }.into())
}

/// Returns the byte range covering the `## {section}` heading and everything
/// under it, up to the next heading of the same or a higher level.
fn find_optional_section_bounds(markdown: &str, section: &str) -> Option<Range<usize>> {
    let headings = collect_headings(markdown);

    let heading_index = headings
        .iter()
        .position(|heading| heading.level == HeadingLevel::H2 && heading.title == section)?;

    let heading = headings.get(heading_index)?;

    let end = headings
        .iter()
        .skip(heading_index.saturating_add(1))
        .find(|candidate| heading_level_number(candidate.level) <= 2)
        .map_or(markdown.len(), |candidate| candidate.range.start);

    Some(heading.range.start..end)
}

fn collect_headings(markdown: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut current_heading: Option<Heading> = None;

    for (event, range) in Parser::new(markdown).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current_heading = Some(Heading { level, title: String::new(), range });
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some(heading) = &mut current_heading {
                    heading.title.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(mut heading) = current_heading.take() {
                    heading.range.end = range.end;
                    headings.push(heading);
                }
            }
            _ => {}
        }
    }

    headings
}

const fn heading_level_number(heading_level: HeadingLevel) -> u8 {
    match heading_level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

#[cfg(test)]
mod tests;
