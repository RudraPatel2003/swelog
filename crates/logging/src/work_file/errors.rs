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

#[derive(Debug, Diagnostic, Error)]
#[error("Managed section {section_id} is missing its end marker")]
#[diagnostic(
    code(swelog::logging::malformed_managed_section),
    help("restore the missing swelog managed-section marker or remove the managed block")
)]
pub struct MalformedManagedSection {
    pub section_id: String,
}
