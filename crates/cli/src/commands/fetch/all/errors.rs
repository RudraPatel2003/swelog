use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("no fetch commands are configured")]
#[diagnostic(
    code(swelog::fetch::no_configured_fetch_sources),
    help("run `swelog fetch status` to see what each fetch command still needs")
)]
pub struct NoConfiguredFetchSources;

#[derive(Debug, Diagnostic, Error)]
#[error("{failed_source_labels} could not be fetched")]
#[diagnostic(
    code(swelog::fetch::fetch_sources_failed),
    help("run the failed fetch commands on their own to see the full error")
)]
pub struct FetchSourcesFailed {
    pub failed_source_labels: String,
}
