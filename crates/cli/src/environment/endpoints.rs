use base_url::base_url::BaseUrl;
use clap::Args;
use updates::registry::DEFAULT_NPM_REGISTRY_BASE_URL;

/// URLs for external services. Overridable for testing.
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
