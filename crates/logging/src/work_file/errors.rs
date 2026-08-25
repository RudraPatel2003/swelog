use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error(
    "Section {section} not found in markdown file. Please ensure '## {section}' exists in the work file"
)]
#[diagnostic(code(swelog::logging::section_not_found))]
pub struct SectionNotFound {
    pub section: String,
}
