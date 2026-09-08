use miette::Result;

use crate::errors::GoogleOAuthApplicationMissing;

const COMPILED_IN_CLIENT_ID: Option<&str> = option_env!("SWELOG_GOOGLE_CLIENT_ID");

const COMPILED_IN_CLIENT_SECRET: Option<&str> = option_env!("SWELOG_GOOGLE_CLIENT_SECRET");

// The secret is not a secret
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoogleOAuthApplication {
    pub client_id: String,
    pub client_secret: String,
}

/// Overrides for the compiled-in Google OAuth client, so a build without one
/// (such as a test build) can still reach a Google-shaped server.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GoogleOAuthApplicationOverrides {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

pub fn get_oauth_application(
    overrides: &GoogleOAuthApplicationOverrides,
) -> Result<GoogleOAuthApplication> {
    let Some(client_id) = resolve_value(overrides.client_id.as_deref(), COMPILED_IN_CLIENT_ID)
    else {
        return Err(GoogleOAuthApplicationMissing.into());
    };

    let Some(client_secret) =
        resolve_value(overrides.client_secret.as_deref(), COMPILED_IN_CLIENT_SECRET)
    else {
        return Err(GoogleOAuthApplicationMissing.into());
    };

    Ok(GoogleOAuthApplication { client_id, client_secret })
}

fn resolve_value(override_value: Option<&str>, compiled_in_value: Option<&str>) -> Option<String> {
    get_non_empty_value(override_value)
        .or_else(|| get_non_empty_value(compiled_in_value))
        .map(ToString::to_string)
}

fn get_non_empty_value(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
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
