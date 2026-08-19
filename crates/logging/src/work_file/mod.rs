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
use errors::{
    MalformedManagedSection,
    SectionNotFound,
};
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

pub fn append_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    content: &str,
    section: &str,
) -> Result<()> {
    update_work_file_from_config(swelog_config, |work_file_content| {
        append_to_section(work_file_content, section, content)
    })
}

pub fn overwrite_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    content: &str,
    section: &str,
) -> Result<()> {
    update_work_file_from_config(swelog_config, |work_file_content| {
        overwrite_section(work_file_content, section, content)
    })
}

pub fn upsert_managed_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    section_id: &str,
    section_title: &str,
    content: &str,
) -> Result<()> {
    update_work_file_from_config(swelog_config, |work_file_content| {
        upsert_managed_section(work_file_content, section_id, section_title, content)
    })
}

pub fn remove_managed_work_file_section_from_config(
    swelog_config: &SwelogConfig,
    section_id: &str,
) -> Result<()> {
    update_work_file_from_config(swelog_config, |work_file_content| {
        remove_managed_section(work_file_content, section_id)
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

fn overwrite_section(markdown: &str, section: &str, content: &str) -> Result<String> {
    let section_bounds = find_section_bounds(markdown, section)?;
    let suffix = slice_from(markdown, section_bounds.end).trim_start_matches('\n');

    let mut result = String::new();

    result.push_str(slice_up_to(markdown, section_bounds.content_start).trim_end_matches('\n'));
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

fn upsert_managed_section(
    markdown: &str,
    section_id: &str,
    section_title: &str,
    content: &str,
) -> Result<String> {
    let managed_block = format_managed_section(section_id, section_title, content);

    if let Some(managed_bounds) = find_managed_section_bounds(markdown, section_id)? {
        return Ok(replace_block(markdown, managed_bounds, &managed_block));
    }

    if let Some(section_bounds) = find_optional_section_bounds(markdown, section_title) {
        return Ok(replace_block(markdown, section_bounds.full, &managed_block));
    }

    let insertion_index = find_optional_section_bounds(markdown, "Log")
        .map_or(markdown.len(), |section_bounds| section_bounds.full.start);

    Ok(insert_block(markdown, insertion_index, &managed_block))
}

fn remove_managed_section(markdown: &str, section_id: &str) -> Result<String> {
    let Some(managed_bounds) = find_managed_section_bounds(markdown, section_id)? else {
        return Ok(markdown.to_string());
    };

    Ok(remove_block(markdown, managed_bounds))
}

fn format_managed_section(section_id: &str, section_title: &str, content: &str) -> String {
    format!(
        "<!-- swelog:managed {section_id}:start -->\n## {section_title}\n{}\n<!-- swelog:managed {section_id}:end -->",
        content.trim_matches('\n')
    )
}

fn insert_block(markdown: &str, insertion_index: usize, block: &str) -> String {
    let prefix = slice_up_to(markdown, insertion_index).trim_end_matches('\n');
    let suffix = slice_from(markdown, insertion_index).trim_start_matches('\n');

    match (prefix.is_empty(), suffix.is_empty()) {
        (true, true) => format!("{block}\n"),
        (true, false) => format!("{block}\n\n{suffix}"),
        (false, true) => format!("{prefix}\n\n{block}\n"),
        (false, false) => format!("{prefix}\n\n{block}\n\n{suffix}"),
    }
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

struct SectionBounds {
    content_start: usize,
    end: usize,
    full: Range<usize>,
}

struct Heading {
    level: HeadingLevel,
    title: String,
    range: Range<usize>,
}

fn find_section_bounds(markdown: &str, section: &str) -> Result<SectionBounds> {
    find_optional_section_bounds(markdown, section)
        .ok_or_else(|| SectionNotFound { section: section.to_string() }.into())
}

fn find_optional_section_bounds(markdown: &str, section: &str) -> Option<SectionBounds> {
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

    Some(SectionBounds { content_start: heading.range.end, end, full: heading.range.start..end })
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

fn find_managed_section_bounds(markdown: &str, section_id: &str) -> Result<Option<Range<usize>>> {
    let start_marker = format!("<!-- swelog:managed {section_id}:start -->");
    let end_marker = format!("<!-- swelog:managed {section_id}:end -->");

    let Some(start_range) = find_exact_line(markdown, &start_marker, 0) else {
        return Ok(None);
    };

    let end_range = find_exact_line(markdown, &end_marker, start_range.end)
        .ok_or_else(|| MalformedManagedSection { section_id: section_id.to_string() })?;

    Ok(Some(start_range.start..end_range.end))
}

fn find_exact_line(
    markdown: &str,
    expected_line: &str,
    start_index: usize,
) -> Option<Range<usize>> {
    let mut line_start = start_index;

    for line in slice_from(markdown, start_index).split_inclusive('\n') {
        let line_end = line_start.saturating_add(line.len());
        let line_without_ending = line.trim_end_matches(['\r', '\n']);

        if line_without_ending == expected_line {
            return Some(line_start..line_end);
        }

        line_start = line_end;
    }

    None
}

#[cfg(test)]
mod tests;
