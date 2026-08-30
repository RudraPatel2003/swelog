use miette::Result;

use crate::errors::GoogleOAuthApplicationMissing;

const COMPILED_IN_CLIENT_ID: Option<&str> = option_env!("SWELOG_GOOGLE_CLIENT_ID");

const COMPILED_IN_CLIENT_SECRET: Option<&str> = option_env!("SWELOG_GOOGLE_CLIENT_SECRET");

// The secret is not a secret
pub struct GoogleOAuthApplication {
    pub client_id: &'static str,
    pub client_secret: &'static str,
}

pub fn get_compiled_in_oauth_application() -> Result<GoogleOAuthApplication> {
    let Some(client_id) = get_non_empty_value(COMPILED_IN_CLIENT_ID) else {
        return Err(GoogleOAuthApplicationMissing.into());
    };

    let Some(client_secret) = get_non_empty_value(COMPILED_IN_CLIENT_SECRET) else {
        return Err(GoogleOAuthApplicationMissing.into());
    };

    Ok(GoogleOAuthApplication { client_id, client_secret })
}

fn get_non_empty_value(compiled_in_value: Option<&'static str>) -> Option<&'static str> {
    compiled_in_value.map(str::trim).filter(|value| !value.is_empty())
}

#[cfg(not(debug_assertions))]
const fn is_compiled_in(compiled_in_value: Option<&str>) -> bool {
    match compiled_in_value {
        Some(value) => !value.is_empty(),
        None => false,
    }
}

// If secrets missing on release, fail the build
#[cfg(not(debug_assertions))]
const _: () = assert!(
    is_compiled_in(COMPILED_IN_CLIENT_ID),
    "release builds need SWELOG_GOOGLE_CLIENT_ID set to a Google Cloud desktop app client ID"
);

#[cfg(not(debug_assertions))]
const _: () = assert!(
    is_compiled_in(COMPILED_IN_CLIENT_SECRET),
    "release builds need SWELOG_GOOGLE_CLIENT_SECRET set to a Google Cloud desktop app client secret"
);
