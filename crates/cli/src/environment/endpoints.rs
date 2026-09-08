use base_url::base_url::BaseUrl;
use clap::Args;
use updates::registry::DEFAULT_NPM_REGISTRY_BASE_URL;

/// Where each external service lives. Overridable so tests can stand in a
/// local server for the real one.
#[derive(Debug, Args)]
pub struct ServiceEndpoints {
    #[arg(
        long = "npm-registry-url",
        global = true,
        env = "SWELOG_NPM_REGISTRY_URL",
        default_value = DEFAULT_NPM_REGISTRY_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub npm_registry: BaseUrl,
}
