mod errors;

use std::fs;

use chrono::NaiveDate;
use config::{
    overwrite::Overwrite,
    setup::{
        default_files::DEFAULT_WORK_FILE_CONTENT,
        swelog_paths::SwelogPaths,
    },
    swelog_config::SwelogConfig,
    utils::{
        ensure_swelog_directory_exists,
        ensure_swelog_file_exists,
    },
};
use dates::formatting::format_date;
use errors::{
    DailyLogAlreadyExists,
    WorkFileNotUpdated,
};
use llm::{
    language_model::LanguageModel,
    prompts::get_daily_log_prompt,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

/// Whether the work file keeps its contents after the summary is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeepWorkFile {
    Yes,
    No,
}

impl KeepWorkFile {
    /// Converts the `--keep` flag clap parsed into the choice it stands for.
    #[must_use]
    pub const fn from_keep_flag(keep: bool) -> Self {
        if keep { Self::Yes } else { Self::No }
    }
}

pub async fn summarize_daily_work_from_config(
    swelog_config: &SwelogConfig,
    language_model: &dyn LanguageModel,
    log_date: &NaiveDate,
    overwrite: Overwrite,
    keep_work_file: KeepWorkFile,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    ensure_swelog_file_exists(&swelog_paths.context_file)?;
    ensure_swelog_file_exists(&swelog_paths.work_file)?;
    ensure_swelog_directory_exists(&swelog_paths.daily_log_directory)?;

    let daily_log_file_name = get_daily_log_file_name(log_date);

    let daily_log_file = swelog_paths.daily_log_directory.join(daily_log_file_name);

    if daily_log_file.exists() && overwrite == Overwrite::No {
        let daily_log_already_exists_error = DailyLogAlreadyExists { daily_log_file };

        return Err(daily_log_already_exists_error.into());
    }

    let context_file_content =
        fs::read_to_string(&swelog_paths.context_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read context file at {}", swelog_paths.context_file.display())
        })?;

    let work_file_content =
        fs::read_to_string(&swelog_paths.work_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read work file at {}", swelog_paths.work_file.display())
        })?;

    if work_file_content == DEFAULT_WORK_FILE_CONTENT {
        let work_file_not_updated_error = WorkFileNotUpdated;

        return Err(work_file_not_updated_error.into());
    }

    let prompt = get_daily_log_prompt(&work_file_content, &context_file_content, log_date);

    let generated_daily_log_content = language_model.generate_response(&prompt).await?;

    let daily_log_content =
        build_daily_log_content(&generated_daily_log_content, &work_file_content);

    fs::write(&daily_log_file, daily_log_content).into_diagnostic().wrap_err_with(|| {
        format!("failed to write daily log file at {}", daily_log_file.display())
    })?;

    if keep_work_file == KeepWorkFile::No {
        fs::write(&swelog_paths.work_file, DEFAULT_WORK_FILE_CONTENT)
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("failed to reset work file at {}", swelog_paths.work_file.display())
            })?;
    }

    Ok(())
}

#[must_use]
pub fn get_daily_log_file_name(log_date: &NaiveDate) -> String {
    let formatted_date = format_date(log_date);

    format!("{formatted_date}.md")
}

fn build_daily_log_content(generated_daily_log_content: &str, work_file_content: &str) -> String {
    let original_notes_content = demote_markdown_headings(work_file_content);

    format!(
        "{}\n\n## Original Notes\n\n{}\n",
        generated_daily_log_content.trim_end(),
        original_notes_content.trim_end()
    )
}

fn demote_markdown_headings(markdown: &str) -> String {
    let mut demoted_lines = Vec::new();

    for line in markdown.lines() {
        demoted_lines.push(demote_markdown_heading(line));
    }

    let mut demoted_markdown = demoted_lines.join("\n");

    if markdown.ends_with('\n') {
        demoted_markdown.push('\n');
    }

    demoted_markdown
}

fn demote_markdown_heading(line: &str) -> String {
    if line.starts_with('#') { format!("##{line}") } else { String::from(line) }
}

#[cfg(test)]
mod tests;
