use std::path::PathBuf;

use chrono::NaiveDate;
use clap::Args;
use credentials::store::CredentialStore;
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_date,
};

use crate::environment::{
    endpoints::ServiceEndpoints,
    update_check::UpdateCheck,
};

#[derive(Debug, Args)]
pub struct GlobalArgs {
    /// Custom path to a swelog config file
    #[arg(long = "config", global = true, env = "SWELOG_CONFIG_FILE", value_name = "PATH")]
    pub config_file_path: Option<PathBuf>,

    /// Custom path to the swelog cache directory. Used for testing.
    #[arg(long, global = true, env = "SWELOG_CACHE_DIRECTORY", value_name = "PATH", hide = true)]
    pub cache_directory: Option<PathBuf>,

    #[arg(long, global = true, env = "SWELOG_CREDENTIAL_STORE", value_name = "STORE", hide = true)]
    pub credential_store: Option<CredentialStore>,

    #[arg(
        long,
        global = true,
        env = "SWELOG_UPDATE_CHECK",
        value_enum,
        default_value_t = UpdateCheck::On,
        hide = true
    )]
    pub update_check: UpdateCheck,

    #[arg(
        long,
        global = true,
        env = "SWELOG_TODAY",
        value_name = DATE_VALUE_NAME,
        value_parser = parse_date,
        hide = true
    )]
    pub today: Option<NaiveDate>,

    #[arg(long, global = true, env = "SWELOG_GOOGLE_CLIENT_ID", value_name = "ID", hide = true)]
    pub google_client_id: Option<String>,

    #[arg(
        long,
        global = true,
        env = "SWELOG_GOOGLE_CLIENT_SECRET",
        value_name = "SECRET",
        hide = true
    )]
    pub google_client_secret: Option<String>,

    #[command(flatten)]
    pub endpoints: ServiceEndpoints,
}
